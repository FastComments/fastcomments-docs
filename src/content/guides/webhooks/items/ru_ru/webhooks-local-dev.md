Для локальной разработки используйте инструмент, такой как [ngrok](https://ngrok.com/).

Чтобы упростить поддержание безопасности системы, локальная разработка следует тому же процессу, что и настройка и защита других окружений. 

### Шаг 1: Добавьте "localhost" в домены в вашем аккаунте.

Добавьте "localhost" [в качестве домена здесь](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Форма добавления домена в настройках аккаунта с введённым localhost в поле имён доменов'; title='Добавить localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Шаг 2: Выберите API‑ключ

Мы собираемся добавить конфигурацию вебхука для вашего домена, поэтому нам понадобится API‑ключ. [Вы можете сделать это здесь.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Форма создания нового API‑секрета с привязанным доменом, установленным на localhost, и ключом с именем Testing'; title='Добавить API‑ключ Testing'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

В разделе «Associate with domain» — выберите ваш домен «localhost».

**ПРИМЕЧАНИЕ: Вместо этого вы можете использовать один API‑секрет для всей тестовой активности и сред предварительного развертывания. Просто добавьте API‑секрет для «All Domains» и дайте ему имя, например, «test».**

Убедитесь, что у вас определён API‑секрет для ваших производственных доменов. События для всех остальных доменов будут использовать wildcard‑секрет (тестовый).

### Шаг 3: Добавьте ваш вебхук

Во время работы ngrok или аналогичного инструмента задайте значение для "localhost" [здесь](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Администрирование вебхуков с выбранным доменом localhost и URL ngrok, заполненным в конечную точку создания комментария'; title='Добавить тестовый вебхук'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

При нажатии `Send Test Payload` мы отправим два тестовых события, чтобы проверить, что вы валидируете API‑ключ.

После успешной проверки нажмите `Save`.

### Шаг 4: Добавьте комментарий

Теперь вы можете добавлять, редактировать или удалять комментарии и должны увидеть, как мы вызываем вашу локальную машину разработки с событиями, используя ваш тестовый API‑ключ. Может быть задержка до 30 секунд, пока события не достигнут вашей машины.

---