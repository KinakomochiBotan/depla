import torch
from torch.utils.data import Dataset
import wthor


class WTHORDataset(Dataset):
    def __init__(self, paths):
        print('load wthor dataset')
        generator = ((torch.from_numpy(data), torch.from_numpy(label)) for (data, label) in wthor.parse(paths))
        self.__train = tuple(generator)

    def __len__(self):
        return self.__train.__len__()

    def __getitem__(self, index):
        return self.__train.__getitem__(index)
