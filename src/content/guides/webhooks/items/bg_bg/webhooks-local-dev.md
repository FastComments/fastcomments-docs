За локална разработка използвайте инструмент като [ngrok](https://ngrok.com/).

За да опрoстим поддържането на системата в сигурно състояние, локалната разработка следва същия процес като настройването и защитата на други среди. 

### Step 1: Add "localhost" to domains in your account.

Добавете "localhost" [като домейн тук](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

Ще добавяме конфигурация за webhook за вашия домейн, така че ще ни трябва API ключ. [Можете да го направите тук.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Под "Associate with domain" - изберете вашия домейн "localhost".

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Уверете се, че имате дефиниран API Secret за вашите продукционни домейни. Събитията за всички останали домейни ще използват wildcard (тестовия) secret.

### Step 3: Add Your Webhook

Докато работи ngrok или подобен инструмент, задайте стойността за "localhost" [тук](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Step 4: Add A Comment

Сега можете да добавяте, редактирате или изтривате коментари и трябва да виждате как ще извикваме вашата локална машина за разработка с тези събития, използвайки вашия тестов API ключ. Може да има до 30 секунди забавяне
за да достигнат събитията до вашата машина.