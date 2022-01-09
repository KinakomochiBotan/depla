import torch
from torch.utils.data import Dataset
import wthor


class WTHORDataset(Dataset):
    def __init__(self, paths, device):
        data = wthor.parse(paths)
        generator = ((torch.from_numpy(data).to(device), torch.from_numpy(label).to(device)) for (data, label) in data)
        self.__train = tuple(generator)

    def __len__(self):
        return len(self.__train)

    def __getitem__(self, item):
        return self.__train[item]


print(len(WTHORDataset(["../wthor/WTH_%d.wtb" % year for year in range(2010, 2021)], "cpu")))
