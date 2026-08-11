Follow the same steps for `localhost` as you would production. Ensure you have production domains and API Secrets setup.

First, navigate to the [panelu administracyjnego webhooków](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Manage Data -> Webhooki.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Strona administracyjna webhooków z selektorem domeny i polem adresu URL punktu końcowego dla każdego zdarzenia komentarza, plus przycisk Wyślij testowy ładunek'; title='Konfiguracja webhooków'; cacheBuster = 'v3' app-screenshot-end]

In this page you can specify endpoints for each type of comment event.

For each type of event, be sure to click Send Test Payload to ensure you've set up your integration correctly. See the next section, „Testowanie”, for details.