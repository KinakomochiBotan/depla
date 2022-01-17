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


class DatasetSetting:
    def __init__(self, start: int, end: int, option: LoadOption):
        self.start = start
        self.end = end
        self.option = option


class AISetting:
    def __init__(self, depth: int, train_dataset: int, validation_dataset: int, epoch: int, parent: bool = False):
        self.depth = depth
        self.train_dataset = train_dataset
        self.validation_dataset = validation_dataset
        self.epoch = epoch
        self.parent = parent


class Experiment(metaclass=ABCMeta):
    def __init__(self, number: int):
        self.__number = number

    @property
    @abstractmethod
    def _datasets(self) -> Tuple[DatasetSetting, ...]:
        pass

    @property
    @abstractmethod
    def _ais(self) -> Tuple[AISetting, ...]:
        pass

    def run(self):
        loaders = tuple(
            DataLoader(
                Dataset(
                    ['wthor/WTH_%d.wtb' % year for year in range(dataset.start, dataset.end + 1)],
                    dataset.option
                ),
                2048,
                True,
                drop_last=True
            ) for dataset in self._datasets
        )

        parent = Path('result/%d' % self.__number)
        device = create_device('cuda')
        ais = self._ais

        for i, setting in enumerate(ais):
            print('AI %d/%d' % (i + 1, len(ais)))
            child = parent / ('.' if setting.parent else str(i + 1))
            child.mkdir(parents=True, exist_ok=True)
            cnn = CNN(device, setting.depth)

            train_loss, train_accuracy, validation_loss, validation_accuracy = Experiment.__run(
                device,
                cnn,
                loaders[setting.train_dataset],
                loaders[setting.validation_dataset],
                setting.epoch
            )

            cnn.save(child / 'cnn.pt')
            Experiment.__save_png(child / 'train_loss.png', train_loss, 400)
            Experiment.__save_png(child / 'train_accuracy.png', train_accuracy, 400)
            Experiment.__save_png(child / 'validation_loss.png', validation_loss, 400)
            Experiment.__save_png(child / 'validation_accuracy.png', validation_accuracy, 400)
            print()

    @staticmethod
    def __run(device, cnn: CNN, train_loader: DataLoader, validation_loader: DataLoader, epoch: int):
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

        return train_loss, train_accuracy, validation_loss, validation_accuracy

    @staticmethod
    def __save_png(path: Path, graph_data, dpi: int):
        figure()
        plot(graph_data)
        savefig(path, dpi=dpi)
