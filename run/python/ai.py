from cnn import CNN
import numpy
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
        for i in range(epoch):
            print('epoch: %d' % i)
            for data, label in loader:
                optimizer.zero_grad()
                criterion(self.__cnn(data.to(self.__device)), label.to(self.__device)).backward()
                optimizer.step()
        self.__cnn.eval()

    def guess(self, player, opponent):
        with torch.no_grad():
            output = self.__cnn(torch.from_numpy(numpy.array([[player, opponent]])).to(self.__device))
            return output.cpu().detach().numpy()[0]
