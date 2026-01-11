Следвайте същите стъпки за `localhost`, както бихте направили за продукционната среда. Уверете се, че имате настроени продукционни домейни и API Secrets.

Първо, отидете на [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Това е достъпно чрез Manage Data -> Webhooks.

Страницата за конфигурация изглежда по следния начин:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

На тази страница можете да посочите крайни точки за всеки тип събитие за коментари.

За всеки тип събитие не забравяйте да натиснете Send Test Payload, за да проверите дали интеграцията ви е настроена правилно. Вижте следващия раздел, "Testing", за подробности.