from numpy import ndarray
import torch
from torch.nn import Module
from torch.utils.data import DataLoader


class AI:
    def __init__(self, net: Module, loader: DataLoader, epoch: int, criterion: Module, optimizer: Module, device):
        self.__device = device
        self.__net = net
        net.train()
        for i in range(epoch):
            print('epoch %d/%d' % (i + 1, epoch))
            for data, label in loader:
                optimizer.zero_grad()
                criterion(net(data.to(device)), label.to(device)).backward()
                optimizer.step()
        net.eval()
        print()

    def guess(self, data: ndarray) -> ndarray:
        with torch.no_grad():
            output = self.__net(torch.from_numpy(data).to(self.__device))
            return output.cpu().detach().numpy()
