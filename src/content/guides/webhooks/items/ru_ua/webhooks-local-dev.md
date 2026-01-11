---
Для локальной разработки используйте инструмент, например [ngrok](https://ngrok.com/).

Чтобы упростить обеспечение безопасности системы, локальная разработка следует тому же процессу, что и настройка и защита других сред. 

### Шаг 1: Добавьте "localhost" в домены в вашей учётной записи.

Добавьте "localhost" [как домен здесь](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Шаг 2: Выберите API-ключ

Мы собираемся добавить конфигурацию webhook для вашего домена, поэтому нам понадобится API-ключ. [Вы можете сделать это здесь.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

В разделе "Associate with domain" - выберите ваш домен "localhost".

**ПРИМЕЧАНИЕ: В качестве альтернативы вы можете использовать один API Secret для всей тестовой активности и промежуточных сред. Просто добавьте API Secret для "All Domains", и дайте ему имя, например "test".**

Убедитесь, что у вас определён API Secret для ваших production-домен(ов). События для всех остальных доменов будут использовать wildcard (testing) секрет.

### Шаг 3: Добавьте ваш Webhook

Запустив ngrok или аналогичный инструмент, установите значение для "localhost" [здесь](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Шаг 4: Добавьте комментарий

Теперь вы можете добавлять, редактировать или удалять комментарии и должны увидеть, что мы вызываем вашу локальную машину разработки с этими событиями, используя ваш тестовый API-ключ. Может быть задержка до 30 секунд
прежде чем события достигнут вашей машины.

---