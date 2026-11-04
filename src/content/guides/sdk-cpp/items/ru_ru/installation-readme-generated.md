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

### Состав библиотеки

Эта библиотека содержит сгенерированный API-клиент и утилиты SSO, упрощающие работу с API.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Публичные и защищённые API

В API-клиенте есть три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие ваш API-ключ, а `PublicApi` содержит методы, которые можно вызывать напрямую из браузера/мобильного устройства и т.д. без аутентификации. `ModerationApi` предоставляет обширный набор быстрых API для модерации в реальном времени. Каждый метод `ModerationApi` принимает параметр `sso` и может выполнять аутентификацию через SSO или cookie сессии FastComments.com.