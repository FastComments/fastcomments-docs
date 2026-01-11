За локални развој, користите алат као што је [ngrok](https://ngrok.com/).

Да бисте поједноставили одржавање безбедности система, локални развој прати исти процес као и подешавање и обезбеђење других окружења. 

### Корак 1: Додајте "localhost" у домене на вашем налогу.

Додајте "localhost" [као домен овде](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Корак 2: Изаберите API кључ

Додаваћемо конфигурацију вебхука за ваш домен, па ће нам бити потребан API кључ. [Можете то урадити овде.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Под "Associate with domain" - изаберите ваш "localhost" домен.

**НАПОМЕНА: Алтернативно, можете користити један API Secret за сву тест активност и staging окружења. Једноставно додајте API Secret за "All Domains", и дајте му име као "test".**

Осигурајте да имате дефинисан API Secret за ваше производне домене. Догађаји за све остале домене користиће wildcard (testing) secret.

### Корак 3: Додајте ваш вебхук

Док покрећете ngrok или сличан алат, подесите вредност за "localhost" [овде](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Корак 4: Додајте коментар

Сада можете додавати, мењати или брисати коментаре и требало би да видите да ћемо позвати вашу локалну машину за развој са догађајима, користећи ваш тестни API кључ. Може бити до 30 секунди кашњења
да догађаји не стигну до ваше машине.