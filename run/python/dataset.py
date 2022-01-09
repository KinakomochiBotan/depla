import torch
from torch.utils.data import Dataset
import wthor
import time


class WTHORDataset(Dataset):
    def __init__(self, paths, device):
        self.__train = [torch.from_numpy(x).to(device) for x in wthor.parse(paths)]

    def __len__(self):
        return len(self.__train)

    def __getitem__(self, item):
        return self.__train[item]


start = time.time()
print(len(WTHORDataset(list(map(lambda year: "run/wthor/WTH_%d.wtb" % year, range(2010, 2021))), "cuda")))
print(time.time() - start)
