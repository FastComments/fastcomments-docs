За локални развој, користите алат као што је [ngrok](https://ngrok.com/).

Да би се поједноставило одржавање безбједности система, локални развој прати исти процес као постављање и обезбјеђивање других окружења. 

### Корак 1: Додајте "localhost" у домене на вашем налогу.

Додајте "localhost" [као домен овдје](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Корак 2: Одаберите API кључ

Додаваћемо конфигурацију webhook-а за ваш домен, тако да ће нам требати API кључ. [Можете то урадити овдје.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Под "Associate with domain" - изаберите ваш "localhost" домен.

**НАПОМЕНА: Као алтернатива, можете користити један API Secret за сву тест активност и staging окружења. Једноставно додајте API Secret за "All Domains", и дајте му име као што је "test".**

Увјерите се да имате дефинисан API Secret за ваше production domain(s). Догађаји за све остале домене користиће wildcard (testing) secret.

### Корак 3: Додајте ваш Webhook

Док покрећете ngrok или сличан алат, подесите вриједност за "localhost" [овдје](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Корак 4: Додајте коментар

Сада можете додавати, уређивати или брисати коментаре и требало би да видите да позивамо вашу локалну развојну машину са догађајима, користећи ваш тестни API кључ. There may be up to 30 seconds delay
for the events to reach your machine.