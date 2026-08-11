For Local development, use a tool like [ngrok](https://ngrok.com/).

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

### Adım 1: Add "localhost" to domains in your account.

Add "localhost" [buradan bir alan adı olarak ekleyin](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Hesap ayarlarında alan adı ekleme formu, alan adı alanına localhost girilmiş'; title='localhost ekle'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Adım 2: Pick an API Key

We're going to be adding webhook configuration for your domain, so we'll need an API key. [Bunu burada yapabilirsiniz.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='İlgili alan adı localhost olarak ayarlanmış ve anahtar adı Testing olan yeni API gizli formu'; title='Test API Anahtarı Ekle'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NOT: Alternatif olarak, tüm test etkinlikleri ve hazırlık ortamları için tek bir API Gizli anahtarı kullanabilirsiniz. "Tüm Alan Adları" için bir API Gizli anahtarı ekleyin ve ona "test" gibi bir ad verin.**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Adım 3: Add Your Webhook

While running ngrok or similar tool, set the value for "localhost" [burada](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhooks yönetim paneli, localhost alan adı seçili ve bir ngrok URL\'si yorum oluşturulan uç noktaya girilmiş'; title='Test Webhook\'u Ekle'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Adım 4: Add A Comment

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API key. There may be up to 30 seconds delay for the events to reach your machine.