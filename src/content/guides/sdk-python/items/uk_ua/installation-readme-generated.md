### PyPI

```bash
pip install fastcomments
```

### Зміст бібліотеки

Ця бібліотека містить два модулі: згенерований клієнт API та основну бібліотеку Python, яка містить написані вручну утиліти, що полегшують роботу з API, включно з підтримкою SSO.

- [Документація клієнтської бібліотеки API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Документація основної бібліотеки, включно з прикладами SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Публічні та захищені API

Для клієнта API існує два класи, `DefaultApi` та `PublicApi`. `DefaultApi` містить методи, що вимагають вашого API-ключа, а `PublicApi` містить виклики API, які можна виконувати безпосередньо з браузера/мобільного пристрою тощо без автентифікації.