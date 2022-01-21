from typing import Union, Tuple, List
from torch import Tensor
from torch.utils.data import Dataset as BaseDataset
from wthor import load


class Dataset(BaseDataset):
    def __init__(
        self,
        paths: Union[Tuple[str], List[str]],
        unique: bool,
        augmentation: bool,
        win: bool,
        draw: bool,
        lose: bool
    ):
        self.__data: Tuple[Tuple[Tensor, int]] = load(paths, unique, augmentation, win, draw, lose)

    def __len__(self):
        return self.__data.__len__()

    def __getitem__(self, x: int) -> Tuple[Tensor, int]:
        return self.__data.__getitem__(x)
