Для локальной разработки используйте инструмент вроде [ngrok](https://ngrok.com/).

Чтобы упростить поддержание безопасности системы, локальная разработка следует тому же процессу, что и настройка и защита других окружений.

### Step 1: Add "localhost" to domains in your account.

Добавьте "localhost" [как домен здесь](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

Нам нужно будет добавить конфигурацию webhook для вашего домена, поэтому понадобится API key. [Вы можете сделать это здесь.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

В разделе "Associate with domain" — выберите ваш домен "localhost".

**ПРИМЕЧАНИЕ: В качестве альтернативы вы можете использовать один API Secret для всей тестовой активности и окружений staging. Просто добавьте API Secret для "All Domains" и дайте ему имя, например "test".**

Убедитесь, что у вас определён API Secret для вашего(их) production домена(ов). События для всех остальных доменов будут использовать wildcard (testing) secret.

### Step 3: Add Your Webhook

Во время работы ngrok или похожего инструмента установите значение для "localhost" [здесь](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

Когда вы нажмёте `Send Test Payload`, мы отправим два тестовых события, чтобы проверить, что вы валидируете API key.

После того как валидация пройдёт, нажмите `Save`.

### Step 4: Add A Comment

Теперь вы можете добавлять, редактировать или удалять комментарии и должны увидеть, что мы вызываем ваш локальный компьютер событиями, используя ваш тестовый API key. Может пройти до 30 секунд, прежде чем события достигнут вашей машины.