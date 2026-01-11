За локална разработка използвайте инструмент като [ngrok](https://ngrok.com/).

За да опростите поддържането на сигурността на системата, локалната разработка следва същия процес като настройването и защитаването на другите среди. 

### Стъпка 1: Добавете "localhost" към домейните в акаунта си.

Добавете "localhost" [като домейн тук](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Стъпка 2: Изберете API ключ

Ще добавим конфигурация за webhook за вашия домейн, затова ще ни трябва API ключ. [Можете да го направите тук.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Стъпка 3: Добавете вашия Webhook

Докато работи ngrok или подобен инструмент, задайте стойността за "localhost" [тук](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Стъпка 4: Добавете коментар

Сега можете да добавяте, редактирате или изтривате коментари и би трябвало да видите повиквания към вашата локална машина за разработка със събитията, използвайки вашия тестов API ключ. Може да има до 30 секунди забавяне
за да достигнат събитията до вашата машина.