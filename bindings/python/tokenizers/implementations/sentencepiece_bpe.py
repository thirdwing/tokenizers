from tokenizers import Tokenizer, AddedToken, pre_tokenizers, decoders, trainers
from tokenizers.models import BPE
from tokenizers.normalizers import NFKC
from .base_tokenizer import BaseTokenizer

from typing import Optional, List, Union


class SentencePieceBPETokenizer(BaseTokenizer):
    """ SentencePiece BPE Tokenizer

    Represents the BPE algorithm, with the pretokenization used by SentencePiece
    """

    def __init__(
        self,
        vocab_file: Optional[str] = None,
        merges_file: Optional[str] = None,
        unk_token: Union[str, AddedToken] = "<unk>",
        replacement: str = "▁",
        add_prefix_space: bool = True,
        dropout: Optional[float] = None,
    ):
        if vocab_file is not None and merges_file is not None:
            tokenizer = Tokenizer(
                BPE.from_files(vocab_file, merges_file, dropout=dropout, unk_token=unk_token)
            )
        else:
            tokenizer = Tokenizer(BPE.empty())

        if tokenizer.token_to_id(str(unk_token)) is not None:
            tokenizer.add_special_tokens([str(unk_token)])

        tokenizer.normalizer = NFKC()
        tokenizer.pre_tokenizer = pre_tokenizers.Metaspace(
            replacement=replacement, add_prefix_space=add_prefix_space
        )
        tokenizer.decoder = decoders.Metaspace(
            replacement=replacement, add_prefix_space=add_prefix_space
        )

        parameters = {
            "model": "SentencePieceBPE",
            "unk_token": unk_token,
            "replacement": replacement,
            "add_prefix_space": add_prefix_space,
            "dropout": dropout,
        }

        super().__init__(tokenizer, parameters)

    def train(
        self,
        files: Union[str, List[str]],
        vocab_size: int = 30000,
        min_frequency: int = 2,
        special_tokens: List[Union[str, AddedToken]] = ["<unk>"],
        limit_alphabet: int = 1000,
        initial_alphabet: List[str] = [],
        show_progress: bool = True,
    ):
        """ Train the model using the given files """

        trainer = trainers.BpeTrainer(
            vocab_size=vocab_size,
            min_frequency=min_frequency,
            special_tokens=special_tokens,
            limit_alphabet=limit_alphabet,
            initial_alphabet=initial_alphabet,
            show_progress=show_progress,
        )
        if isinstance(files, str):
            files = [files]
        self._tokenizer.train(trainer, files)
