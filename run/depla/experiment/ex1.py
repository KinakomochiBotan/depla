import torch
from torch.nn import CrossEntropyLoss
from torch.optim import SGD
from torch.utils.data import DataLoader
from .. import CNN, Depth, Dataset, train, get_path, wthor_paths, save_net, save_png
from wthor import LoadOption


def ex1():
    device = torch.device("cuda")
    cnn = CNN(Depth.Four, device)
    wthor_path = 'wthor/WTH_%d.wtb'
    train_data = Dataset(wthor_paths(wthor_path, 2010, 2018), LoadOption().win().draw().lose())
    train_loader = DataLoader(train_data, 2048, True, drop_last=True)
    validation_data = Dataset(wthor_paths(wthor_path, 2019, 2019), LoadOption().win().draw().lose())
    validation_loader = DataLoader(validation_data, 2048, True, drop_last=True)
    criterion = CrossEntropyLoss()
    optimizer = SGD(cnn.parameters(), 0.01, 0.95, weight_decay=0.0005)

    train_loss, train_accuracy, validation_loss, validation_accuracy = train(
        cnn,
        4,
        train_loader,
        validation_loader,
        criterion,
        optimizer,
        device
    )

    result_pass = 'result/ex1'
    save_net(get_path(result_pass, 'cnn.pt'), cnn)

    save_png(
        get_path(result_pass, 'loss.png'),
        'loss',
        train_loss,
        'train',
        validation_loss,
        'validation',
        400
    )

    save_png(
        get_path(result_pass, 'accuracy.png'),
        'accuracy',
        train_accuracy,
        'train',
        validation_accuracy,
        'validation',
        400
    )
