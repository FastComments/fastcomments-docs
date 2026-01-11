Suivez les mêmes étapes pour `localhost` que pour la production. Assurez-vous d'avoir configuré des domaines de production et des API Secrets.

First, navigate to the [Administration des webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Gérer les données -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

On this page you can specify endpoints for each type of comment event.

For each type of event, be sure to click Send Test Payload to ensure you've set up your integration correctly. See the next section, "Tests", for details.

---