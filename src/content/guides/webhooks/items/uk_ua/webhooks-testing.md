У розділі адміністрування Webhooks є кнопки `Send Test Payload` для кожного типу події (Create, Update, Delete). Події Create і Update надсилають тестовий об'єкт WebhookComment, а при тестуванні Delete буде надіслано тестове тіло запиту, що містить лише ID.

## Перевірка коректності payload'ів

Під час тестування інтеграції webhook переконайтеся, що вхідні запити містять наступні заголовки:

1. **`token`** - Ваш API Secret
2. **`X-FastComments-Timestamp`** - Unix-мітка часу (у секундах)
3. **`X-FastComments-Signature`** - HMAC-SHA256 підпис

Використовуйте перевірку підпису HMAC, щоб переконатися, що payload'и автентичні.

## Інструменти для тестування

Ви можете використовувати інструменти, такі як [webhook.site](https://webhook.site) або [ngrok](https://ngrok.com), щоб переглядати вхідні payload'и webhook під час розробки.

## Типи подій

- **Create Event**: Викликається, коли створюється новий коментар. Default method: PUT
- **Update Event**: Викликається, коли коментар редагується. Default method: PUT
- **Delete Event**: Викликається, коли коментар видаляється. Default method: DELETE

Кожна подія містить повні дані коментаря в тілі запиту (див. [Структури даних](/guides/webhooks/webhooks-structures) щодо формату payload).