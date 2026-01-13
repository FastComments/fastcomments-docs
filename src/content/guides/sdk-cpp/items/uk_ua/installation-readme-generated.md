---
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

Ця бібліотека містить згенерований клієнт API та утиліти SSO, щоб полегшити роботу з API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Публічні та захищені API

Для клієнта API існують два класи, `DefaultAPI` та `PublicAPI`. `DefaultAPI` містить методи, що вимагають вашого API-ключа, а `PublicAPI` містить запити до API, які можна виконувати безпосередньо з браузера/мобільного пристрою тощо без аутентифікації.
---