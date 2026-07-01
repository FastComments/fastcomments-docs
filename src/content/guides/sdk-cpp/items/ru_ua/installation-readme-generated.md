### Install Dependencies

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Building from Source

```bash
mkdir build
cd build
cmake ..
make
```

### Installing

```bash
sudo make install
```

### Library Contents

Эта библиотека содержит сгенерированный клиент API и утилиты SSO, упрощающие работу с API.

- [Документация библиотеки клиента API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Public vs Secured APIs

Для клиента API существует три класса, `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие вашего API‑ключа, а `PublicApi` содержит
методы, которые можно вызывать напрямую из браузера/мобильного устройства/и т.д. без аутентификации. `ModerationApi` предоставляет обширный набор живых и быстрых API модерации. Каждый `ModerationApi` метод принимает параметр `sso` и может аутентифицироваться через SSO или куки сессии FastComments.com.