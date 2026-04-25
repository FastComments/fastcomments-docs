---
All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Задължително

- **Tenant ID** - Вашият FastComments Tenant ID. Намира се в [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Необходимо за Secure SSO, проверка на webhook и синхронизация на страници. Намира се в [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Стил на коментиране

Изберете уиджета, който отговаря на начина, по който искате хората да общуват на вашия сайт.

- **Live Comments** - Коментари с нишки в реално време.
- **Streaming Chat** - Интерактивен чат на живо, подходящ за събития и излъчвания на живо.
- **Collab Chat** - Анотации чрез маркиране на текст в основното съдържание. Посетителите маркират текст и стартират дискусия в контекста.
- **Collab Chat + Comments** - Както collab chat, така и стандартни коментари на една и съща страница.

## Режим SSO

- **None** - Без SSO. Потребителите коментират като гости или създават акаунт във FastComments.
- **Simple** - Предава информация за потребителя в Drupal (име, имейл, аватар) към FastComments без верификация от страна на сървъра.
- **Secure** - Използва HMAC-SHA256 за верификация на потребителите от Drupal с FastComments. Препоръчва се, когато имате конфигуриран API Secret.

See the `Single Sign-On (SSO)` section for details.

## Други настройки

- **CDN URL** - По подразбиране: `https://cdn.fastcomments.com`.
- **Site URL** - По подразбиране: `https://fastcomments.com`.
- **Email notifications** - Изпраща имейл до автора на съдържанието, когато е публикуван нов коментар по тяхното съдържание.

For EU data residency, see the `EU Data Residency` section.

---