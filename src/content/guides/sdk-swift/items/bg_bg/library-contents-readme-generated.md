FastComments Swift SDK се състои от няколко модула:

- **Client Module** - Автоматично генериран API клиент за FastComments REST APIs
  - Пълни дефиниции на типовете за всички API модели
  - Както автентикирани (`DefaultAPI`), така и публични (`PublicAPI`) крайни точки
  - Пълна поддръжка на async/await
  - Вижте [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) за подробна документация на API

- **SSO Module** - Помощни средства за Single Sign-On от страна на сървъра
  - Сигурно генериране на токени за потребителска автентикация
  - Поддръжка както на прост, така и на защитен режим на SSO
  - Подписване на токени, базирано на HMAC-SHA256, използвайки CryptoKit