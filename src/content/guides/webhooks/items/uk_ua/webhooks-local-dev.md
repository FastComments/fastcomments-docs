Для локальної розробки використовуйте інструмент на кшталт [ngrok](https://ngrok.com/).

Щоб спростити підтримку безпеки системи, локальна розробка слідує тому ж процесу, що й налаштування та захист інших середовищ.

### Step 1: Add "localhost" to domains in your account.

Додайте "localhost" [як домен тут](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

Ми будемо додавати конфігурацію webhook для вашого домену, тому нам знадобиться API key. [Ви можете зробити це тут.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Під «Associate with domain» — виберіть ваш домен "localhost".

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Переконайтесь, що у вас визначено API Secret для ваших продукційних доменів. Події для всіх інших доменів використовуватимуть загальний (тестовий) секрет.

### Step 3: Add Your Webhook

Поки запущено ngrok або подібний інструмент, встановіть значення для "localhost" [тут](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, ми надішлемо два тестові події, щоб перевірити, що ви валідируєте API key.

Після успішної валідації натисніть `Save`.

### Step 4: Add A Comment

Тепер ви можете додавати, редагувати або видаляти коментарі та повинні побачити, як ми викликаємо вашу локальну машину розробника цими подіями, використовуючи ваш тестовий API key. Може бути затримка до 30 секунд
щоб події дісталися до вашої машини.

---