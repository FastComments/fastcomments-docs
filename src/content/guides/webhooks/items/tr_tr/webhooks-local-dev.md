For Local development, use a tool like [ngrok](https://ngrok.com/).

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

### Adım 1: Hesabınızdaki alanlara "localhost" ekleyin.

Add "localhost" [as a domain here](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='localhost ekle'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Adım 2: Bir API Secret Seçin

We're going to be adding webhook configuration for your domain, so we'll need an API secret. [You can do that here.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Test API Secret Ekle'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NOT: Alternatif olarak, tüm test etkinlikleri ve hazırlık (staging) ortamları için tek bir API Secret kullanabilirsiniz. Basitçe "All Domains" için bir API Secret ekleyin ve ona "test" gibi bir isim verin.**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Adım 3: Webhook'unuzu ekleyin

While running ngrok or similar tool, set the value for "localhost" [here](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Test Webhook Ekle'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API secret.

Once it validates, hit `Save`.

### Adım 4: Bir Yorum Ekleyin

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API secret. There may be up to 30 seconds delay
for the events to reach your machine.

---