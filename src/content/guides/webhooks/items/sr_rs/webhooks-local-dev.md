За локални развој, користите алат као што је [ngrok](https://ngrok.com/).

Да би се олакшало одржавање безбедности система, локални развој прати исти процес као постављање и обезбеђивање других окружења. 

### Корак 1: Додајте „localhost“ у домене у вашем налогу.

Додајте „localhost“ [као домен овде](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Форма за додавање домена у подешавањима налога са унетим localhost у поље за имена домена'; title='Додај localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Корак 2: Одаберите API кључ

Додаћемо конфигурацију вебхука за ваш домен, па нам је потребан API кључ. [То можете урадити овде.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Нова форма за API тајну са повезаним доменом постављеним на localhost и кључем названим Testing'; title='Додајте API кључ Testing'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Под „Associate with domain“ - изаберите ваш домен „localhost“.

**НАПОМЕНА: Алтернативно, можете користити једну API тајну за све тестирање и сценичка окружења. Само додајте API тајну за „All Domains“ и дајте јој име као што је „test“.**

Уверите се да имате дефинисану API тајну за ваше продукционе домене. Догађаји за све друге домене користиће wildcard (тест) тајну.

### Корак 3: Додајте ваш вебхук

Док користите ngrok или сличан алат, поставите вредност за „localhost“ [овде](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Администрација вебхукова са изабраним доменом localhost и унетим ngrok URL-ом у крајњи пункт за креирање коментара'; title='Додајте тестни вебхук'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Када кликнете `Send Test Payload`, послаћемо два тестна догађаја да проверимо да ли сте валидарали API кључ.

Када се валидација успе, кликните `Save`.

### Корак 4: Додајте коментар

Сада можете да додајете, уређујете или бришете коментаре и требало би да видите да ваш локални развојни рачунар прима догађаје, користећи ваш тестни API кључ. Може постоји до 30 секунди кашњења док догађаји стигну до вашег рачунара.