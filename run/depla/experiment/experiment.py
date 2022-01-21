from typing import Tuple, List
from datetime import datetime, timedelta
from pathlib import Path
from torch import device as create_device, no_grad
from torch.nn import CrossEntropyLoss
from torch.optim import SGD
from torch.utils.data import DataLoader
from matplotlib.pyplot import figure, plot, savefig
from colosseum import random as vs_random
from .. import CNN, Dataset


class Experiment:
    def __init__(self, path: str, dataset: Tuple[int, int, bool, bool, bool, bool, bool], ai: Tuple[int, int]):
        self.__path = path
        self.__dataset = dataset
        self.__ai = ai

    def run(self):
        train_loader = Experiment.__loader(
            self.__dataset[0],
            self.__dataset[1],
            self.__dataset[2],
            self.__dataset[3],
            self.__dataset[4],
            self.__dataset[5],
            self.__dataset[6],
            True
        )

        test_loader_1 = Experiment.__loader(
            2020,
            2020,
            True,
            self.__dataset[3],
            self.__dataset[4],
            self.__dataset[5],
            self.__dataset[6],
            False
        )

        test_loader_2 = Experiment.__loader(
            2020,
            2020,
            True,
            True,
            True,
            True,
            True,
            False
        )

        path = Path('result')
        path /= self.__path.replace('.', '/')
        path.mkdir(parents=True, exist_ok=True)
        file = open(path / 'result.txt', 'w')
        device = create_device('cuda')
        cnn = CNN(device, self.__ai[0])
        print('train')

        time, loss_history, accuracy_history = Experiment.__train(
            device,
            cnn,
            train_loader,
            self.__ai[1]
        )

        cnn.save(path / 'cnn.pt')
        file.write('time: %s\n' % time)
        Experiment.__save_png(path / 'loss.png', loss_history, 400)
        Experiment.__save_png(path / 'accuracy.png', accuracy_history, 400)
        print("test1")
        file.write('test1: %f\n' % Experiment.__test(device, cnn, test_loader_1))
        print("test2")
        file.write('test2: %f\n' % Experiment.__test(device, cnn, test_loader_2))
        print('vs random player')
        file.write('AI: %d, draw: %d, random: %d)\n' % vs_random(cnn, 10000))
        file.close()

    @staticmethod
    def __loader(start: int, end: int, unique: bool, augmentation: bool, win: bool, draw: bool, lose: bool, drop: bool):
        return DataLoader(
            Dataset(
                ['wthor/WTH_%d.wtb' % year for year in range(start, end + 1)],
                unique,
                augmentation,
                win,
                draw,
                lose
            ),
            2048,
            True,
            drop_last=drop
        )

    @staticmethod
    def __train(device, cnn: CNN, loader: DataLoader, epoch: int) -> Tuple[timedelta, List[float], List[float]]:
        criterion = CrossEntropyLoss()
        optimizer = SGD(cnn.parameters(), 0.01, 0.95, weight_decay=0.0005)
        loss_history = []
        accuracy_history = []
        start = datetime.now()

        for i in range(epoch):
            print('epoch %d/%d' % (i + 1, epoch))
            cnn.train()

            for data, label in loader:
                optimizer.zero_grad()
                output = cnn(data.to(device))
                label = label.to(device)
                loss = criterion(output, label)
                loss.backward()
                optimizer.step()
                loss_history.append(loss.item())
                answers = output.max(1)[1] == label
                accuracy_history.append(answers.sum().item() / answers.size()[0])

        end = datetime.now()
        cnn.eval()
        return end - start, loss_history, accuracy_history

    @staticmethod
    def __test(device, cnn: CNN, loader: DataLoader):
        total = 0
        correct = 0
        cnn.eval()

        with no_grad():
            for data, label in loader:
                answers = cnn(data.to(device)).max(1)[1] == label.to(device)
                total += answers.size()[0]
                correct += answers.sum().item()

        return correct / total

    @staticmethod
    def __save_png(path: Path, graph_data, dpi: int):
        figure()
        plot(graph_data)
        savefig(path, dpi=dpi)
