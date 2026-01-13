### Установка зависимостей

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Сборка из исходников

```bash
mkdir build
cd build
cmake ..
make
```

### Установка

```bash
sudo make install
```

### Содержимое библиотеки

Эта библиотека содержит сгенерированный клиент API и утилиты SSO, чтобы упростить работу с API.

- [Документация библиотеки клиента API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Публичные и защищённые API

Для клиента API существуют два класса, `DefaultAPI` и `PublicAPI`. `DefaultAPI` содержит методы, которые требуют ваш API-ключ, а `PublicAPI` содержит вызовы API, которые можно выполнять непосредственно из браузера/мобильного устройства/etc без аутентификации.