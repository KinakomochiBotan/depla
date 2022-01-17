from typing import Union, BinaryIO, IO
from os import PathLike
from numpy import ndarray
from torch import no_grad, from_numpy, save, load
from torch import Tensor
from torch.nn import Module, Sequential, Conv2d, BatchNorm2d, ReLU, Flatten


class CNN(Module):
    def __init__(self, device, depth: int):
        super(CNN, self).__init__()
        self.__device = device
        self.__depth = depth
        sequential = [Conv2d(2, 128, 1, device=device), BatchNorm2d(128, device=device), ReLU()]

        for _ in range(0, depth):
            sequential += [Conv2d(128, 128, 3, padding=1, device=device), BatchNorm2d(128, device=device), ReLU()]

        sequential += [Conv2d(128, 1, 1, device=device), Flatten()]
        self.__block = Sequential(*sequential)

    def forward(self, x: Tensor) -> Tensor:
        return self.__block(x)

    def guess(self, x: ndarray) -> ndarray:
        with no_grad():
            return self(from_numpy(x).to(self.__device)).cpu().detach().numpy()

    def save(self, path: Union[str, PathLike, BinaryIO, IO[bytes]]):
        save({
            'depth': self.__depth,
            'state_dict': self.state_dict()
        }, path)

    @classmethod
    def load(cls, path, device):
        data = load(path)
        cnn = CNN(device, data['depth'])
        cnn.load_state_dict(data['state_dict'])
        cnn.eval()
        return cnn
