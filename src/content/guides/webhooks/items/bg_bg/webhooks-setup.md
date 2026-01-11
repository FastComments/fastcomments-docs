Следвайте същите стъпки за `localhost` както бихте направили за production. Уверете се, че сте настроили production домейни и API Secrets.

Първо, отидете на [Администриране на Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). Това е достъпно чрез Manage Data -> Webhooks.

Страницата за конфигурация изглежда по следния начин:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

На тази страница можете да посочите endpoints за всеки тип събитие на коментар.

За всеки тип събитие задължително кликнете върху Send Test Payload, за да се уверите, че интеграцията е настроена правилно. Вижте следващия раздел, "Testing", за подробности.