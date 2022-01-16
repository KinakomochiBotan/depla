from torch import Tensor
from torch.utils.data import Dataset as BaseDataset
from wthor import parse, LoadOption


class Dataset(BaseDataset):
    def __init__(self, paths: [str], option: LoadOption):
        print('load wthor dataset')
        self.__data: ((Tensor, int),) = parse(paths, option)

    def __len__(self) -> int:
        return self.__data.__len__()

    def __getitem__(self, x: int) -> (Tensor, int):
        return self.__data.__getitem__(x)
