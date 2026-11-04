За локална разработка използвайте инструмент като [ngrok](https://ngrok.com/).

За да опростим поддържането на системата сигурна, локалната разработка следва същия процес като настройването и осигуряването на сигурност на други среди.

### Стъпка 1: Добавете "localhost" към домейните в акаунта си.

Добавете "localhost" [като домейн тук](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Формата за добавяне на домейн в настройките на акаунта с въведен localhost в полето за имена на домейни'; title='Добавяне на localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Стъпка 2: Изберете API ключ

Ще добавяме конфигурация за webhook за вашия домейн, така че ще ни трябва API ключ. [Можете да направите това тук.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Форма за нов API секрет със свързания домейн, зададен на localhost, и ключ с име Testing'; title='Добавяне на тестов API ключ'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Под "Associate with domain" - изберете вашия "localhost" домейн.

**ЗАБЕЛЕЖКА: Алтернативно можете да използвате един API Secret за цялата тестова дейност и стейджинг среди. Просто добавете API Secret за "All Domains" и му дайте име като "test".**

Уверете се, че имате дефиниран API Secret за вашия производствен(и) домейн(и). Събитията за всички останали домейни ще използват wildcard (тестовия) секрет.

### Стъпка 3: Добавете вашия Webhook

Докато ngrok или подобен инструмент работи, задайте стойността за "localhost" [тук](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Администрация на webhooks с избран localhost домейн и ngrok URL, въведен в endpoint-а за създаден коментар'; title='Добавяне на тестов webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

При кликване върху `Send Test Payload` ще изпратим два тестови събития, за да проверим дали валидирате API ключа.

След като валидацията премине, натиснете `Save`.

### Стъпка 4: Добавете коментар

Сега можете да добавяте, редактирате или изтривате коментари и трябва да видите как извикваме вашата локална машина за разработка със събитията, използвайки вашия тестов API ключ. Може да има закъснение до 30 секунди
преди събитията да достигнат вашата машина.