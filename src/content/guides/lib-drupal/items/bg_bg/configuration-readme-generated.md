Отидете до **Администрация > Конфигурация > Съдържание > FastComments** (`/admin/config/content/fastcomments`).

### Настройки

- **Tenant ID** (задължително) - Вашият FastComments Tenant ID. Намерете го под [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Изисква се за Secure SSO, потвърждение на webhook и синхронизация на страници. Намерява се под [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Интеграция за Single Sign-On:
  - **None** - Без SSO; потребителите коментират като гости или създават FastComments акаунти.
  - **Simple** - Предава информация за потребителя от Drupal (име, имейл, аватар) на FastComments без верификация на сървъра.
  - **Secure** - Използва HMAC-SHA256 верификация за сигурна автентикация на Drupal потребители с FastComments (препоръчително).
- **Commenting Style** - Типът на джаджата за показване:
  - **Live Comments** - Нишкови коментари в реално време.
  - **Streaming Chat** - Интерфейс за чат в реално време.
  - **Collab Chat** - Съвместно анотиране чрез избор на текст върху основното съдържание.
  - **Collab Chat + Comments** - Комбинация от Collab чат и стандартни коментари.
- **CDN URL** - FastComments CDN URL (по подразбиране: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL (по подразбиране: `https://fastcomments.com`).
- **Email notifications** - Изпраща имейл на авторите на съдържанието, когато бъде публикуван нов коментар в тяхното съдържание.

### Adding Comments to Content Types

Добавете полето **FastComments** към вашите типове съдържание чрез **Структура > Типове съдържание > [type] > Управление на полета**. Полето има превключвател за статус и опционален персонализиран идентификатор за всеки ентитет.

### EU Data Residency

За резиденция на данни в ЕС актуализирайте:
- **CDN URL** на `https://cdn-eu.fastcomments.com`
- **Site URL** на `https://eu.fastcomments.com`