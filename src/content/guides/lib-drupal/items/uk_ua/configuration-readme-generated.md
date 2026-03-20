Навігація до **Адміністрування > Конфігурація > Вміст > FastComments** (`/admin/config/content/fastcomments`).

### Налаштування

- **Tenant ID** (обов'язково) - Ваш Tenant ID FastComments. Знайдіть це в розділі [Налаштування > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЄС](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Необхідно для Secure SSO, перевірки webhook та синхронізації сторінок. Знаходиться в розділі [Налаштування > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЄС](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Інтеграція Single Sign-On:
  - **None** - Без SSO, користувачі коментують як гості або створюють облікові записи FastComments.
  - **Simple** - Передає інформацію користувача Drupal (ім'я, email, аватар) до FastComments без серверної перевірки.
  - **Secure** - Використовує перевірку HMAC-SHA256 для безпечної автентифікації користувачів Drupal у FastComments (рекомендується).
- **Commenting Style** - Тип віджета для відображення:
  - **Live Comments** - Коментарі в реальному часі, впорядковані у нитки.
  - **Streaming Chat** - Інтерфейс живого чату.
  - **Collab Chat** - Спільна анотація вибраного тексту на головній області вмісту.
  - **Collab Chat + Comments** - І collab chat, і стандартні коментарі.
- **CDN URL** - URL CDN FastComments (за замовчуванням: `https://cdn.fastcomments.com`).
- **Site URL** - URL сайту FastComments (за замовчуванням: `https://fastcomments.com`).
- **Email notifications** - Надсилати електронний лист авторам вмісту, коли на їхній вміст додається новий коментар.

### Додавання коментарів до типів вмісту

Додайте поле **FastComments** до ваших типів вмісту через **Структура > Типи вмісту > [type] > Керувати полями**. Поле має перемикач статусу та необов'язковий власний ідентифікатор для кожного об'єкта.

### Розміщення даних у ЄС

Для розміщення даних у ЄС оновіть:
- **CDN URL** на `https://cdn-eu.fastcomments.com`
- **Site URL** на `https://eu.fastcomments.com`