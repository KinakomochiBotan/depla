from cnn import CNN
import torch
from torch.nn import CrossEntropyLoss
from torch.optim import SGD


class AI:
    def __init__(self):
        self.__device = torch.device("cuda")
        self.__cnn = CNN(self.__device)
        self.__criterion = CrossEntropyLoss()
        self.__optimizer = SGD(self.__cnn.parameters(), lr=0.0001, momentum=0.9, weight_decay=0.005)

    def tensor(self, data):
        return torch.tensor(data, device=self.__device)

    def guess(self, player, opponent):
        self.__optimizer.zero_grad()
        return self.__cnn(self.tensor([[player, opponent]])).tolist()[0]

    def train(self, data, label):
        self.__criterion(self.__cnn(self.tensor(data)), self.tensor(label)).backward()
        self.__optimizer.step()


class AITrainer:
    def __init__(self, data):
        self.__ai = AI()
        self.__data = list(map(self.__ai.tensor, data))
