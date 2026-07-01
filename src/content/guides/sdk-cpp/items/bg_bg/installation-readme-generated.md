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

Тази библиотека съдържа генерирания API клиент и SSO инструментите, които улесняват работата с API-то.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Public vs Secured APIs

За API клиента има три класа, `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа методи, които могат да бъдат извикани директно от браузър/мобилно устройство и т.н. без автентикация. `ModerationApi` предоставя обширен набор от живи и бързи API за модериране. Всеки метод на `ModerationApi` приема параметър `sso` и може да се автентицира чрез SSO или бисквитка за сесия от FastComments.com.