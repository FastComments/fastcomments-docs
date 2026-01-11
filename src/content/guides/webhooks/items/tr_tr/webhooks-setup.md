`localhost` için, production'da uyguladığınız aynı adımları izleyin. production alan adlarının ve API Secrets'in yapılandırıldığından emin olun.

First, navigate to the [Webhooks yönetimi](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Manage Data -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

In this page you can specify endpoints for each type of comment event.

For each type of event, be sure to click Send Test Payload to ensure you've set up your integration correctly. See the next section, "Testing", for details.