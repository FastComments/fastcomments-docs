FastComments Swift SDK се състои от няколко модула:

- **Client Module** - API клиент за FastComments REST APIs
  - Пълни дефиниции на типовете за всички API модели
  - Автентифицирани (`DefaultAPI`), публични (`PublicAPI`), и модерационни (`ModerationAPI`) методи
  - Пълна поддръжка на async/await
  - Вижте [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) за подробна документация на API-то

- **SSO Module** - Инструменти за Single Sign-On от страна на сървъра
  - Сигурно генериране на токени за удостоверяване на потребители
  - Поддръжка както на прост, така и на сигурен режим на SSO
  - Подписване на токени на базата на HMAC-SHA256 с помощта на CryptoKit