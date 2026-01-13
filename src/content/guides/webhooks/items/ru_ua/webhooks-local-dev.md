Для локальной разработки используйте инструмент вроде [ngrok](https://ngrok.com/).

Чтобы упростить обеспечение безопасности системы, локальная разработка следует тому же процессу, что и настройка и защита других окружений. 

### Шаг 1: Добавьте "localhost" в домены в вашей учётной записи.

Добавьте "localhost" [как домен здесь](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Шаг 2: Выберите API-ключ

Мы собираемся добавить конфигурацию вебхука для вашего домена, поэтому нам потребуется API-ключ. [Вы можете сделать это здесь.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

В поле "Associate with domain" - выберите ваш домен "localhost".

**ПРИМЕЧАНИЕ: В качестве альтернативы вы можете использовать один API Secret для всей тестовой активности и окружений staging. Просто добавьте API Secret для "All Domains", и дайте ему имя, например "test".**

Убедитесь, что у вас определён API Secret для ваших production-доменов. События для всех остальных доменов будут использовать wildcard (тестовый) секрет.

### Шаг 3: Добавьте ваш вебхук

Во время запуска ngrok или аналогичного инструмента установите значение для "localhost" [здесь](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

При нажатии `Send Test Payload`, мы отправим два тестовых события, чтобы проверить, что вы валидируете API-ключ.

После подтверждения нажмите `Save`.

### Шаг 4: Добавьте комментарий

Теперь вы можете добавлять, редактировать или удалять комментарии и должны увидеть, как мы вызываем вашу локальную машину разработки с событиями, используя ваш тестовый API-ключ. Может быть до 30 секунд задержки
прежде чем события достигнут вашей машины.