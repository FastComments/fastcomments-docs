Для локальної розробки використовуйте такий інструмент, як [ngrok](https://ngrok.com/).

Щоб спростити підтримку безпеки системи, локальна розробка проходить той самий процес налаштування та захисту, що й інші середовища.

### Крок 1: Додайте "localhost" до доменів у вашому акаунті.

Додайте "localhost" [як домен тут](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Форма додавання домену в налаштуваннях акаунта з введеним localhost у полі доменних імен'; title='Додати localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Крок 2: Виберіть API-ключ

Ми будемо додавати конфігурацію вебхука для вашого домену, тому нам знадобиться API-ключ. [Ви можете зробити це тут.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Форма нового API-секрету з доменом localhost та ключем з назвою Testing'; title='Додати тестовий API-ключ'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

У полі "Associate with domain" — виберіть ваш домен "localhost".

**ПРИМІТКА: Крім того, ви можете використовувати один API Secret для всіх тестових дій і проміжних середовищ. Просто додайте API Secret для "All Domains" і дайте йому назву на кшталт "test".**

Переконайтеся, що у вас визначено API Secret для вашого виробничого домену (доменів). Події для всіх інших доменів використовуватимуть wildcard (тестовий) секрет.

### Крок 3: Додайте свій вебхук

Поки запущено ngrok або подібний інструмент, встановіть значення для "localhost" [тут](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Адмін-панель вебхуків з вибраним доменом localhost та URL ngrok, заповненим у полі endpoint створення коментаря'; title='Додати тестовий вебхук'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Коли ви натиснете `Send Test Payload`, ми надішлемо дві тестові події, щоб перевірити, чи ви валідуєте API-ключ.

Після успішної валідації натисніть `Save`.

### Крок 4: Додайте коментар

Тепер ви можете додавати, редагувати або видаляти коментарі, і ви маєте побачити, як ми викликаємо вашу локальну машину розробки з подіями, використовуючи ваш тестовий API-ключ. Може бути затримка до 30 секунд
перед тим, як події досягнуть вашої машини.