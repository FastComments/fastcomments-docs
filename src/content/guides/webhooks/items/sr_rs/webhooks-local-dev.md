---
За локални развој, користите алат као што је [ngrok](https://ngrok.com/).

Да би се поједноставило одржавање безбедности система, локални развој следи исти процес као и подешавање и обезбеђивање других окружења. 

### Корак 1: Додајте "localhost" у домене на вашем налогу.

Додајте "localhost" [као домен овде](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Корак 2: Изаберите API Key

Додаваћемо конфигурацију webhook-а за ваш домен, па ће нам требати API key. [То можете урадити овде.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**НАПОМЕНА: Алтернативно, можете користити један API Secret за сву тест активност и staging окружења. Једноставно додате API Secret за "All Domains", и дајте му име као што је "test".**

Обавезно имате дефинисан API Secret за ваше production домене. Догађаји за све остале домене ће користити wildcard (testing) secret.

### Корак 3: Додајте ваш webhook

Док покрећете ngrok или сличан алат, подесите вредност за "localhost" [овде](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

Када кликнете на `Send Test Payload`, послаћемо два тест догађаја да проверимо да ли валидарате API key.

Када се валидација заврши, притисните `Save`.

### Корак 4: Додајте коментар

Сада можете додавати, уређивати или брисати коментаре и требало би да видите да позивамо вашу локалну развојну машину са догађајима, користећи ваш testing API key. Може бити до 30 секунди кашњења
за догађаје да стигну до ваше машине.

---