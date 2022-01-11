from typing import List
from torch.utils.data import Dataset as BaseDataset
import wthor


class Dataset(BaseDataset):
    def __init__(self, paths: List[str]):
        print('load wthor dataset')
        self.__data = wthor.parse(paths)

    def __len__(self):
        return self.__data.__len__()

    def __getitem__(self, x: int):
        return self.__data.__getitem__(x)
