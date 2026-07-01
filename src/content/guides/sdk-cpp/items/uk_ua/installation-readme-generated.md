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

Ця бібліотека містить згенерований клієнт API та утиліти SSO, які спрощують роботу з API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Public vs Secured APIs

Для клієнта API є три класи, `DefaultApi`, `PublicApi` та `ModerationApi`. `DefaultApi` містить методи, які вимагають ваш API‑ключ, а `PublicApi` містить методи, які можна викликати безпосередньо з браузера/мобільного пристрою тощо без автентифікації. `ModerationApi` надає широкий набір живих та швидких API модерації. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватися за допомогою SSO або кукі сесії FastComments.com.