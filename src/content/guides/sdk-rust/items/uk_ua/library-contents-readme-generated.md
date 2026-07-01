FastComments Rust SDK складається з кількох модулів:

- **Client Module** - API‑клієнт для FastComments REST API
  - Повні типові визначення для всіх моделей API
  - Три API‑клієнти, які охоплюють всі методи FastComments:
    - `default_api` (**DefaultApi**) - методи, автентифіковані API‑ключем, для використання на сервері
    - `public_api` (**PublicApi**) - публічні методи без API‑ключа, безпечні для виклику з браузерів та мобільних додатків
    - `moderation_api` (**ModerationApi**) - великий набір живих та швидких API модерації. Кожен метод Moderation приймає параметр `sso` і може автентифікуватися через SSO або cookie сесії FastComments.com.
  - Повна підтримка async/await з tokio
  - Дивіться [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) для докладної документації API

- **SSO Module** - Утиліти серверного Single Sign-On
  - Безпечне генерування токенів для автентифікації користувачів
  - Підтримка як простих, так і захищених режимів SSO
  - Підписання токенів на основі HMAC‑SHA256

- **Core Types** - Спільні визначення типів та утиліти
  - Моделі коментарів і структури метаданих
  - Конфігурації користувачів та орендарів
  - Допоміжні функції для поширених операцій