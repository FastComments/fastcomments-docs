本番環境と同じ手順で`localhost`にも設定してください。本番用ドメインとAPI Secretsが設定されていることを確認してください。

First, navigate to the [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Manage Data -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

In this page you can specify endpoints for each type of comment event.

For each type of event, be sure to click Send Test Payload to ensure you've set up your integration correctly. See the next section, "Testing", for details.

---