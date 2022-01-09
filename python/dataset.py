from torch.utils.data import Dataset
import wthor
import time


class WTHORDataset(Dataset):
    def __init__(self, paths):
        self.__train = wthor.parse(paths)

    def __len__(self):
        return len(self.__train)

    def __getitem__(self, item):
        return self.__train[item]


start = time.time()
print(len(WTHORDataset(list(map(lambda year: "../wthor_data/WTH_%d.wtb" % year, range(2010, 2021))))))
print(time.time() - start)
