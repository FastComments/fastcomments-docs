Усі налаштування знаходяться в `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Required

- **Tenant ID** - Ваш FastComments Tenant ID. Знайдіть це в [Налаштування > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Потрібно для Secure SSO, перевірки вебхуків і синхронізації сторінок. Знайдете в [Налаштування > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Commenting Style

Виберіть віджет, що відповідає тому, як ви хочете, щоб люди спілкувалися на вашому сайті.

- **Live Comments** - Потокові коментарі в режимі реального часу з відповідями (threaded).
- **Streaming Chat** - Інтерфейс живого чату, підходить для подій та трансляцій.
- **Collab Chat** - Анотації на вибраному тексті в основній області контенту. Відвідувачі виділяють текст і починають обговорення в контексті.
- **Collab Chat + Comments** - Одночасно collab chat і стандартні коментарі на одній сторінці.

## SSO Mode

- **None** - Без SSO. Користувачі коментують як гості або створюють обліковий запис FastComments.
- **Simple** - Передає інформацію користувача Drupal (ім'я, електронна пошта, аватар) до FastComments без перевірки на стороні сервера.
- **Secure** - Використовує HMAC-SHA256 для перевірки користувачів Drupal у FastComments. Рекомендується, якщо у вас налаштовано API Secret.

Дивіться розділ `Single Sign-On (SSO)` для деталей.

## Other Settings

- **CDN URL** - За замовчуванням `https://cdn.fastcomments.com`.
- **Site URL** - За замовчуванням `https://fastcomments.com`.
- **Email notifications** - Надсилати електронний лист автору контенту, коли на його матеріалі з'являється новий коментар.

Для розміщення даних у ЄС див. розділ `EU Data Residency`.