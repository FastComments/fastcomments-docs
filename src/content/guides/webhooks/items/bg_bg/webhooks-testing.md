В админ панела за Webhooks има бутони `Send Test Payload` за всеки тип събитие (Create, Update, Delete). Събитията Create и Update изпращат фиктивен обект WebhookComment, докато при тест на Delete ще бъде изпратено фиктивно тяло на заявката, съдържащо само ID.

## Проверка на payload-ите

Когато тествате интеграцията на вашия webhook, уверете се, че входящите заявки включват следните заглавки:

1. **`token`** - Your API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (seconds)
3. **`X-FastComments-Signature`** - HMAC-SHA256 signature

Използвайте проверката на HMAC подписа, за да се уверите, че payload-ите са автентични.

## Инструменти за тестване

Можете да използвате инструменти като [webhook.site](https://webhook.site) или [ngrok](https://ngrok.com) за да инспектирате входящите webhook payload-ове по време на разработка.

## Типове събития

- **Create Event**: Се задейства, когато е създаден нов коментар. По подразбиране метод: PUT
- **Update Event**: Се задейства, когато коментарът бъде редактиран. По подразбиране метод: PUT
- **Delete Event**: Се задейства, когато коментарът бъде изтрит. По подразбиране метод: DELETE

Всяко събитие включва пълните данни на коментара в тялото на заявката (виж [Структури от данни](/guide-webhooks.html#webhooks-structures) за формата на payload-а).

---