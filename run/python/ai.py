from cnn import OthelloCNN
from dataset import WTHORDataset
import torch
from torch.nn import CrossEntropyLoss
from torch.optim import SGD
from torch.utils.data import DataLoader


class AI:
    def __init__(self):
        self.__device = torch.device('cuda')
        self.__cnn = OthelloCNN(self.__device)
        self.__cnn.train()
        dataset = WTHORDataset(['run/wthor/WTH_%d.wtb' % year for year in range(2010, 2021)])
        loader = DataLoader(dataset, 2048, True, drop_last=True)
        criterion = CrossEntropyLoss()
        optimizer = SGD(self.__cnn.parameters(), lr=0.0001, momentum=0.9, weight_decay=0.005)
        for i in range(16):
            print('epoch: %d' % i)
            for data, label in loader:
                optimizer.zero_grad()
                criterion(self.__cnn(data.to(self.__device)), label.to(self.__device)).backward()
                optimizer.step()
        self.__cnn.eval()

    def guess(self, data):
        with torch.no_grad():
            output = self.__cnn(torch.from_numpy(data).to(self.__device))
            return output.cpu().detach().numpy()
