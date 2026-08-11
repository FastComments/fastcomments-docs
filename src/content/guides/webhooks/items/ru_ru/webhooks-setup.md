---
Follow the same steps for `localhost` as you would production. Ensure you have production domains and API Secrets setup.

First, navigate to the [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Manage Data -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Страница администрирования вебхуков с селектором домена и полем URL конечной точки для каждого события комментария, плюс Send Test Payload'; title='Конфигурация вебхуков'; cacheBuster = 'v3' app-screenshot-end]

In this page you can specify endpoints for each type of comment event.

For each type of event, be sure to click Send Test Payload to ensure you've set up your integration correctly. See the next section, "Testing", for details.

---