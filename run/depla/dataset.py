from typing import List, Tuple
from torch import Tensor
from torch.utils.data import Dataset as BaseDataset
import wthor


class Dataset(BaseDataset):
    def __init__(self, paths: List[str]):
        print('load wthor dataset')
        self.__data: Tuple[(Tensor, float)] = wthor.parse(paths)

    def __len__(self) -> int:
        return self.__data.__len__()

    def __getitem__(self, x: int) -> (Tensor, float):
        return self.__data.__getitem__(x)
