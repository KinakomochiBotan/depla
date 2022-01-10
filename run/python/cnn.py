from torch.nn import Module, Conv2d, BatchNorm2d, Linear
from torch.nn.functional import relu


class OthelloCNN(Module):
    def __init__(self, device):
        super(OthelloCNN, self).__init__()
        self.__conv1 = Conv2d(2, 128, 3, padding=1, device=device)
        self.__batch1 = BatchNorm2d(128, device=device)
        self.__conv2 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch2 = BatchNorm2d(128, device=device)
        self.__conv3 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch3 = BatchNorm2d(128, device=device)
        self.__conv4 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch4 = BatchNorm2d(128, device=device)
        self.__conv5 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch5 = BatchNorm2d(128, device=device)
        self.__conv6 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch6 = BatchNorm2d(128, device=device)
        self.__conv7 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch7 = BatchNorm2d(128, device=device)
        self.__conv8 = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch8 = BatchNorm2d(128, device=device)
        self.__linear1 = Linear(128 * 64, 128 * 8, device=device)
        self.__linear2 = Linear(128 * 8, 64, device=device)

    def forward(self, x):
        x = relu(self.__batch1(self.__conv1(x)))
        x = relu(self.__batch2(self.__conv2(x)))
        x = relu(self.__batch3(self.__conv3(x)))
        x = relu(self.__batch4(self.__conv4(x)))
        x = relu(self.__batch5(self.__conv5(x)))
        x = relu(self.__batch6(self.__conv6(x)))
        x = relu(self.__batch7(self.__conv7(x)))
        x = relu(self.__batch8(self.__conv8(x)))
        x = x.flatten(1)
        x = self.__linear1(x)
        x = relu(x)
        x = self.__linear2(x)
        x = x.view(-1, 8, 8)
        x = x.softmax(1)
        return x
