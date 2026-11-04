### Инсталиране на зависимости

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Компилиране от изходния код

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

Тази библиотека съдържа генерирания API клиент и SSO помощни средства, които улесняват работата с API.

- [Документация на API клиентската библиотека](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Публични срещу защитени API

За API клиента има три класа: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа методи, които могат да се извикват директно от браузър/мобилно устройство и т.н. без автентикация. `ModerationApi` предоставя обширен набор от API за жива и бърза модерация. Всеки метод на `ModerationApi` приема параметър `sso` и може да се автентикира чрез SSO или чрез сесийна бисквитка на FastComments.com.