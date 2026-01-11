Для локальної розробки використовуйте інструмент, такий як [ngrok](https://ngrok.com/).

Щоб спростити підтримання безпеки системи, локальна розробка використовує той самий процес, що й налаштування та захист інших середовищ. 

### Крок 1: Додайте "localhost" до доменів у вашому обліковому записі.

Додайте "localhost" [як домен тут](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Крок 2: Виберіть API-ключ

Ми збираємося додати конфігурацію webhook для вашого домену, тому нам знадобиться API-ключ. [Ви можете зробити це тут.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

У розділі "Associate with domain" — виберіть ваш домен "localhost".

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Переконайтеся, що для ваших production-доменів визначено API Secret. Події для всіх інших доменів використовуватимуть wildcard (тестовий) секрет.

### Крок 3: Додайте ваш webhook

Під час запуску ngrok або подібного інструменту встановіть значення для "localhost" [тут](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

Коли ви натиснете `Send Test Payload`, ми надішлемо дві тестові події, щоб перевірити, чи ви валідируєте API-ключ.

Після валідації натисніть `Save`.

### Крок 4: Додайте коментар

Тепер ви можете додавати, редагувати або видаляти коментарі, і ви маєте бачити, як ми надсилаємо запити до вашої локальної машини розробки з цими подіями, використовуючи ваш тестовий API-ключ. Може бути затримка до 30 секунд, поки події дійдуть до вашої машини.

---