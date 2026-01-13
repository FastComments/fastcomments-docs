### Инсталиране на зависимости

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Изграждане от изходния код

```bash
mkdir build
cd build
cmake ..
make
```

### Инсталиране

```bash
sudo make install
```

### Съдържание на библиотеката

Тази библиотека съдържа генерирания API клиент и SSO инструментите, за да улесни работата с API-то.

- [Документация на библиотеката за API клиента](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Публични срещу защитени API

За API клиента има два класа, `DefaultAPI` и `PublicAPI`. Класът `DefaultAPI` съдържа методи, които изискват вашия API ключ, а `PublicAPI` съдържа извиквания на API
които могат да се извършат директно от браузър/мобилно устройство/и т.н. без удостоверяване.