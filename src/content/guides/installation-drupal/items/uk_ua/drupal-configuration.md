All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Обов’язкові

- **Tenant ID** – Ваш FastComments Tenant ID. Знайдіть його в [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** – Потрібний для Secure SSO, перевірки вебхуків та синхронізації сторінок. Знаходиться в [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Зберігання API Secret

Модуль читає API Secret з першого з цих джерел, яке має значення:

1. **Key module** – Встановіть [Key](https://www.drupal.org/project/key), створіть ключ *Authentication* і виберіть його в **API Secret key** у формі налаштувань. Ключ може бути підкріплений змінною середовища або файлом поза кореневим каталогом веб‑сайту.
2. **settings.php** – Додайте `$config['fastcomments.settings']['api_secret'] = 'your-secret';` у `settings.php` або `settings.local.php`. Секрет ніколи не потрапляє у експортовану конфігурацію.
3. **The settings form** – Введіть його у поле **API Secret**. Це зберігає значення в `fastcomments.settings`, тому воно потрапляє у експорт конфігурації. Звіт стану Drupal попереджає, коли це активне джерело.

Форма показує, яке джерело активне. Якщо залишити поле порожнім, збережений секрет залишиться. Позначте **Remove the API Secret stored in configuration**, щоб видалити його після переміщення секрету до модуля Key або `settings.php`.

## Стиль коментування

Виберіть віджет, який відповідає тому, як ви хочете, щоб люди спілкувалися на вашому сайті.

- **Live Comments** – Коментарі в реальному часі з деревовидною структурою.
- **Streaming Chat** – Інтерфейс живого чату, підходить для подій та прямих трансляцій.
- **Collab Chat** – Анотація виділеного тексту в основній області контенту. Відвідувачі виділяють текст і починають обговорення в контексті.
- **Collab Chat + Comments** – І чат‑колаборація, і стандартні коментарі на одній сторінці.

## Режим SSO

- **None** – Без SSO. Користувачі коментують як гості або створюють обліковий запис FastComments.
- **Simple** – Передає інформацію про користувача Drupal (ім'я, електронна пошта, аватар) у FastComments без серверної верифікації.
- **Secure** – Використовує HMAC‑SHA256 для перевірки користувачів Drupal у FastComments. Рекомендовано, коли у вас налаштовано API Secret.

Дивіться розділ `Single Sign-On (SSO)` для деталей.

## Інші налаштування

- **CDN URL** – За замовчуванням `https://cdn.fastcomments.com`.
- **Site URL** – За замовчуванням `https://fastcomments.com`.
- **Email notifications** – Надіслати електронний лист автору контенту, коли новий коментар розміщено під його матеріалом.

Для резидентності даних в ЄС дивіться розділ `EU Data Residency` section.

---