За локални развој, користите алат као што је [ngrok](https://ngrok.com/).

Да би се поједноставило одржавање сигурности система, локални развој прати исти процес као и подешавање и обезбеђивање других окружења. 

### Корак 1: Додајте "localhost" у домене на вашем налогу.

Додајте "localhost" [као домен овде](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Корак 2: Изаберите API кључ

Додаваћемо конфигурацију вебхука за ваш домен, па нам треба API кључ. [То можете урадити овде.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

У пољу "Associate with domain" - изаберите ваш домен "localhost".

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Корак 3: Додајте ваш вебхук

Док покрећете ngrok или сличан алат, подесите вредност за "localhost" [овде](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Корак 4: Додајте коментар

Сада можете додавати, уређивати или брисати коментаре и требало би да видите позиве ка вашој локалној машини за развој са догађајима, користећи ваш тест API кључ. Може доћи до застоја до 30 секунди пре него што догађаји стигну до ваше машине.

---