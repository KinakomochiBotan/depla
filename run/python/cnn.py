from torch.nn import Module, Conv2d, MaxPool2d, Linear
from torch.nn.functional import relu, softmax


class CNN(Module):
    def __init__(self, device):
        super(CNN, self).__init__()
        self.__conv11 = Conv2d(2, 128, 3, padding=1, device=device)
        self.__conv12 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__conv13 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__pool1 = MaxPool2d(3, 1)
        self.__conv21 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__conv22 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__conv23 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__pool2 = MaxPool2d(3, 1)
        self.__linear1 = Linear(128 * 16, 128 * 2, device=device)
        self.__linear2 = Linear(128 * 2, 64, device=device)
        self.__softmax = softmax

    def forward(self, x):
        x = relu(self.__conv11(x))
        x = relu(self.__conv12(x))
        x = relu(self.__conv13(x))
        x = self.__pool1(x)
        x = relu(self.__conv21(x))
        x = relu(self.__conv22(x))
        x = relu(self.__conv23(x))
        x = self.__pool2(x)
        x = x.flatten(1)
        x = self.__linear1(x)
        x = self.__linear2(x)
        x = softmax(x)
        x = x.view(-1, 8, 8)
        return x
