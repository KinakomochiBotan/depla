import torch
from torch.utils.data import Dataset, DataLoader
import wthor


class WTHORDataset(Dataset):
    def __init__(self, device, paths):
        data = wthor.parse(paths)
        generator = ((torch.from_numpy(data).to(device), torch.from_numpy(label).to(device)) for (data, label) in data)
        self.__train = tuple(generator)

    def __len__(self):
        return len(self.__train)

    def __getitem__(self, item):
        return self.__train[item]
