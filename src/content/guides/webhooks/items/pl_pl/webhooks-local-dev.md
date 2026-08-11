---
For Local development, use a tool like [ngrok](https://ngrok.com/).

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

### Krok 1: Dodaj „localhost” do domen w swoim koncie.

Add "localhost" [as a domain here](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Formularz dodawania domeny w ustawieniach konta z wprowadzonym localhost w polu nazw domen'; title='Dodaj localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Krok 2: Wybierz klucz API

We're going to be adding webhook configuration for your domain, so we'll need an API key. [You can do that here.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Nowy formularz tajnego klucza API z powiązaną domeną ustawioną na localhost i kluczem o nazwie Testing'; title='Dodaj klucz API testowy'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**UWAGA: Alternatywnie możesz używać jednego tajnego klucza API dla całej aktywności testowej i środowisk staging. Po prostu dodaj tajny klucz API dla "All Domains", i nadaj mu nazwę, np. "test".**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Krok 3: Dodaj swój webhook

While running ngrok or similar tool, set the value for "localhost" [here](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Panel administracyjny webhooków z wybraną domeną localhost i wypełnionym adresem URL ngrok w endpointzie utworzenia komentarza'; title='Dodaj webhook testowy'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Krok 4: Dodaj komentarz

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API key. There may be up to 30 seconds delay
for the events to reach your machine.