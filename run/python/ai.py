from cnn import CNN
import torch
from torch.nn import CrossEntropyLoss
from torch.optim import SGD


class AI:
    def __init__(self, device, epoch, loader):
        self.__device = device
        self.__cnn = CNN(device)
        criterion = CrossEntropyLoss()
        optimizer = SGD(self.__cnn.parameters(), lr=0.0001, momentum=0.9, weight_decay=0.005)
        self.__cnn.train()
        for _ in range(epoch):
            for data, label in loader:
                optimizer.zero_grad()
                criterion(self.__cnn(data), label).backward()
                optimizer.step()
        self.__cnn.eval()

    def guess(self, player, opponent):
        with torch.no_grad():
            return self.__cnn(torch.tensor([[player, opponent]], device=self.__device)).tolist()[0]
