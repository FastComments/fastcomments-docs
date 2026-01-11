---
За локални развој, користите алат као што је [ngrok](https://ngrok.com/).

Да би се поједноставило одржавање сигурности система, локални развој прати исти процес као и подешавање и обезбеђивање других окружења. 

### Корак 1: Додајте "localhost" у домене на вашем налогу.

Додајте "localhost" [као домен овде](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Корак 2: Изаберите API кључ

Додаћемо конфигурацију webhook-а за ваш домен, па нам треба API кључ. [Можете то урадити овде.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Под "Associate with domain" - изаберите ваш "localhost" домен.

**НАПОМЕНА: Као алтернатива, можете користити један API Secret за све тест активности и стагинг окружења. Једноставно додајте API Secret за "All Domains", и дајте му име као "test".**

Уверите се да имате дефинисан API Secret за ваше продукционе домене. Догађаји за све остале домене ће користити wildcard (тестни) секрет.

### Корак 3: Додајте ваш webhook

Док покрећете ngrok или сличан алат, подесите вредност за "localhost" [овде](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, poslaćemo dva тест догађаја да проверимо да ли потврђујете валидност API кључа.

Once it validates, hit `Save`.

### Корак 4: Додајте коментар

Сада можете додавати, уређивати или брисати коментаре и требало би да видите како шаљемо захтеве ка вашем локалном развојном рачунару са догађајима, користећи ваш тестни API кључ. Може бити до 30 секунди кашњења
за догађаје да стигну до вашег рачунара.

---