For Local development, use a tool like [ngrok](https://ngrok.com/).

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

### Стъпка 1: Добавете „localhost“ към домейните във вашия акаунт.

Add "localhost" [as a domain here](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Формулярът за добавяне на домейн в настройките на акаунта с въведено localhost в полето за имена на домейни'; title='Добавяне на localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Стъпка 2: Изберете API ключ

We're going to be adding webhook configuration for your domain, so we'll need an API key. [You can do that here.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Нов формуляр за API тайна с асоциирания домейн зададен на localhost и ключа с име Testing'; title='Добавяне на API ключ за тестване'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**ЗАБЕЛЕЖКА: Алтернативно, можете да използвате една API тайна за цялата тестова активност и среди за предварително тестване. Просто добавете API тайна за „All Domains“ и я наречете например „test“.**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Стъпка 3: Добавете вашия уебхук

While running ngrok or similar tool, set the value for "localhost" [here](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Администратор на уебхукове с избран домейн localhost и попълнен ngrok URL в крайна точка за създаден коментар'; title='Добавяне на тестов уебхук'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Стъпка 4: Добавете коментар

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API key. There may be up to 30 seconds delay
for the events to reach your machine.