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

Эта библиотека содержит сгенерированный клиент API и утилиты SSO, которые упрощают работу с API.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Публичные и защищённые API

Для клиентской библиотеки API существуют два класса: `DefaultAPI` и `PublicAPI`. `DefaultAPI` содержит методы, для которых требуется ваш API key, а `PublicAPI` содержит api calls, которые можно выполнять непосредственно из браузера/мобильного устройства и т.д. без аутентификации.