### Встановлення залежностей

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Збірка з вихідного коду

```bash
mkdir build
cd build
cmake ..
make
```

### Встановлення

```bash
sudo make install
```

### Вміст бібліотеки

Ця бібліотека містить згенерований клієнт API та утиліти SSO, щоб спростити роботу з API.

- [Документація клієнтської бібліотеки API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Публічні та захищені API

Для клієнта API є три класи: `DefaultApi`, `PublicApi` та `ModerationApi`. `DefaultApi` містить методи, які потребують ваш API-ключ, а `PublicApi` містить методи, які можна викликати безпосередньо з браузера/мобільного пристрою тощо без автентифікації. `ModerationApi` надає розширений набір швидких API для модерації в реальному часі. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватися через SSO або cookie сесії FastComments.com.