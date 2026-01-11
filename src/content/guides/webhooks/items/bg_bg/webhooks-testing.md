В админ панела за Webhooks има бутони `Send Test Payload` за всеки тип събитие (Create, Update, Delete). Събитията Create и Update изпращат фиктивен обект WebhookComment, докато при тестване на Delete ще бъде изпратено фиктивно тяло на заявката, съдържащо само ID.

## Проверка на payload-ите

When testing your webhook integration, verify the incoming requests include the following headers:

1. **`token`** - Вашият API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (в секунди)
3. **`X-FastComments-Signature`** - HMAC-SHA256 подпис

Използвайте проверката на HMAC подписа, за да се уверите, че payload-ите са автентични.

## Инструменти за тестване

Можете да използвате инструменти като [webhook.site](https://webhook.site) или [ngrok](https://ngrok.com), за да инспектирате входящите webhook payloads по време на разработка.

## Типове събития

- **Create Event**: Активира се когато се създаде нов коментар. По подразбиране метод: PUT
- **Update Event**: Активира се когато коментарът бъде редактиран. По подразбиране метод: PUT
- **Delete Event**: Активира се когато коментарът бъде изтрит. По подразбиране метод: DELETE

Each event includes the full comment data in the request body (see [Структури с данни](/guides/webhooks/webhooks-structures) for the payload format).