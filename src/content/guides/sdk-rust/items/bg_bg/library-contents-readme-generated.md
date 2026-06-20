The FastComments Rust SDK се състои от няколко модула:

- **Client Module** - API клиент за FastComments REST API-та
  - Пълни дефиниции на типове за всички модели на API-то
  - Три API клиента, покриващи всички методи на FastComments:
    - `default_api` (**DefaultApi**) - методи, удостоверени с API ключ за използване от страна на сървъра
    - `public_api` (**PublicApi**) - публични методи без API ключ, безопасни за извикване от браузъри и мобилни приложения
    - `moderation_api` (**ModerationApi**) - методи, поддържащи таблото на модератора, включително модериране на коментари (list, count, search, logs, export), модераторски действия (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), бани (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), и значки & доверие (award/remove badges, manual badges, get/set trust factor, user internal profile). Всеки Moderation метод приема параметър `sso`, така че повикването да може да бъде направено от името на SSO-автентикиран модератор.
  - Пълна поддръжка на async/await с tokio
  - Вижте [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) за подробна документация на API-то

- **SSO Module** - Сървърни помощни средства за Single Sign-On
  - Сигурно генериране на токени за удостоверяване на потребители
  - Поддръжка както на прост, така и на сигурен режим на SSO
  - Подписване на токени на базата на HMAC-SHA256

- **Core Types** - Споделени дефиниции на типове и помощни средства
  - Модели на коментари и структури за метаданни
  - Конфигурации за потребители и наематели
  - Помощни функции за често срещани операции