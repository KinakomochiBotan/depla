from typing import List, Tuple
from torch import Tensor
from torch.utils.data import Dataset as BaseDataset
from wthor import LoadOption, parse


class Dataset(BaseDataset):
    def __init__(self, paths: List[str], option: LoadOption):
        print('load wthor dataset')
        self.__data: Tuple[Tuple[Tensor, int]] = parse(paths, option)

    def __len__(self) -> int:
        return self.__data.__len__()

    def __getitem__(self, x: int) -> Tuple[Tensor, int]:
        return self.__data.__getitem__(x)
