Follow the same steps for `localhost` as you would production. Ensure you have production domains and API Secrets setup.

First, navigate to the [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Manage Data -> Webhooks.

The page lists every webhook on your account:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks admin page listing each webhook with its URL, event, domain, method, status and queued event count'; title='Webhooks List'; cacheBuster = 'v4' app-screenshot-end]

Click **New Webhook** to add one. Each webhook has a URL, one comment event (created, updated or deleted), a domain, and an HTTP method:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='New webhook form with URL, event, domain and HTTP method fields plus Send Test Payload'; title='New Webhook'; cacheBuster = 'v4' app-screenshot-end]

Every webhook is delivered independently. You can send the same event to several endpoints, and a webhook scoped
to **All Domains** receives comments from every domain even when a domain-specific webhook exists for the same event.
The same URL, event and domain cannot be added twice.

Before saving, click **Send Test Payload** to check the endpoint accepts a signed request. See the next section, "Testing", for details.

From the list you can edit, disable, re-enable or delete a webhook. Disabling keeps queued events until the webhook is re-enabled; deleting discards them.

Webhooks can also be created through the API, for example by Zapier. Those appear in the same list with the source **API**. See Managing Webhooks via the API.
