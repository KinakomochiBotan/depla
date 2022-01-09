import torch
from torch.nn import Module, Conv2d, MaxPool2d, Linear
from torch.nn.functional import relu, softmax


class CNN(Module):
    def __init__(self, device):
        super(CNN, self).__init__()
        self.__relu = relu
        self.__conv11 = Conv2d(2, 128, 3, padding=1, device=device)
        self.__conv12 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__conv13 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__pool1 = MaxPool2d(2, 2)
        self.__conv21 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__conv22 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__conv23 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__pool2 = MaxPool2d(2, 2)
        self.__conv31 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__conv32 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__conv33 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__pool3 = MaxPool2d(2, 2)
        self.__flatten = lambda x: torch.flatten(x, 1)
        self.__linear = Linear(128 * 8, 64, device=device)
        self.__softmax = softmax

    def forward(self, x):
        x = self.__relu(self.__conv11(x))
        x = self.__relu(self.__conv12(x))
        x = self.__relu(self.__conv13(x))
        x = self.__pool1(x)
        x = self.__relu(self.__conv21(x))
        x = self.__relu(self.__conv22(x))
        x = self.__relu(self.__conv23(x))
        x = self.__pool2(x)
        x = self.__relu(self.__conv31(x))
        x = self.__relu(self.__conv32(x))
        x = self.__relu(self.__conv33(x))
        x = self.__pool3(x)
        x = self.__flatten(x)
        x = self.__linear(x)
        x = self.__softmax(x)
        return x
