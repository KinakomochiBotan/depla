from abc import ABCMeta, abstractmethod
from typing import Tuple
from pathlib import Path
from torch import device as create_device, no_grad
from torch.nn import CrossEntropyLoss
from torch.optim import SGD
from torch.utils.data import DataLoader
from matplotlib.pyplot import figure, plot, savefig
from wthor import LoadOption
from .. import CNN, Dataset


class Experiment(metaclass=ABCMeta):
    def __init__(self, path: str):
        self.__path = path

    @property
    @abstractmethod
    def _dataset(self) -> Tuple[int, int, bool, bool, bool, bool, bool]:
        pass

    @property
    @abstractmethod
    def _ai(self) -> Tuple[int, int]:
        pass

    def run(self):
        dataset = self._dataset
        test_loader = Experiment.__loader(dataset[0], dataset[1], )

        loaders = tuple(
            DataLoader(
                Dataset(
                    ['wthor/WTH_%d.wtb' % year for year in range(start, end + 1)],
                    option
                ),
                2048,
                True,
                drop_last=True
            ) for start, end, option in zip(self._datasets, [True, True, False])
        )

        path = Path('result')
        path /= self.__path
        path.mkdir(parents=True, exist_ok=True)
        device = create_device('cuda')
        depth, epoch = self._ai
        cnn = CNN(device, depth)

        train_loss, train_accuracy, validation_loss, validation_accuracy = Experiment.__train(
            device,
            cnn,
            loaders[0],
            loaders[1],
            epoch
        )

        cnn.save(path / 'cnn.pt')
        Experiment.__save_png(path / 'train_loss.png', train_loss, 400)
        Experiment.__save_png(path / 'train_accuracy.png', train_accuracy, 400)
        Experiment.__save_png(path / 'validation_loss.png', validation_loss, 400)
        Experiment.__save_png(path / 'validation_accuracy.png', validation_accuracy, 400)
        print()

    @staticmethod
    def __loader(start: int, end: int, option: LoadOption, drop: bool):
        return DataLoader(
            Dataset(
                ['wthor/WTH_%d.wtb' % year for year in range(start, end + 1)],
                option
            ),
            2048,
            True,
            drop_last=drop
        )

    @staticmethod
    def __train(device, cnn: CNN, train_loader: DataLoader, validation_loader: DataLoader, epoch: int):
        criterion = CrossEntropyLoss()
        optimizer = SGD(cnn.parameters(), 0.01, 0.95, weight_decay=0.0005)
        train_loss = []
        train_accuracy = []
        validation_loss = []
        validation_accuracy = []

        for i in range(epoch):
            print('epoch %d/%d' % (i + 1, epoch))
            cnn.train()

            for data, label in train_loader:
                optimizer.zero_grad()
                output = cnn(data.to(device))
                label = label.to(device)
                loss = criterion(output, label)
                loss.backward()
                optimizer.step()
                train_loss.append(loss.item())
                accuracy = output.max(1)[1] == label
                train_accuracy.append(accuracy.sum().item() / accuracy.size()[0])

            cnn.eval()

            with no_grad():
                for data, label in validation_loader:
                    output = cnn(data.to(device))
                    label = label.to(device)
                    validation_loss.append(criterion(output, label).item())
                    accuracy = output.max(1)[1] == label
                    validation_accuracy.append(accuracy.sum().item() / accuracy.size()[0])

        cnn.eval()
        return train_loss, train_accuracy, validation_loss, validation_accuracy

    @staticmethod
    def __test(device, cnn: CNN, test_loader: DataLoader):
        total = 0
        correct = 0
        cnn.eval()

        with no_grad():
            for data, label in test_loader:
                accuracy = cnn(data.to(device)).max(1)[1] == label.to(device)
                total += accuracy.size()[0]
                correct += accuracy.sum().item()

        return correct / total

    @staticmethod
    def __save_png(path: Path, graph_data, dpi: int):
        figure()
        plot(graph_data)
        savefig(path, dpi=dpi)
