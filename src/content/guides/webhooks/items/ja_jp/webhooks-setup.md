---
`localhost` に対しても本番と同じ手順を実行してください。プロダクション用ドメインと API シークレットが設定されていることを確認してください。

まず、[Webhooks 管理](https://fastcomments.com/auth/my-account/manage-data/webhooks) に移動します。これは「Manage Data」→「Webhooks」からアクセスできます。

このページにはアカウントにあるすべてのWebhookが一覧表示されます:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='各WebhookのURL、イベント、ドメイン、メソッド、ステータス、キューされたイベント数を一覧表示するWebhooks管理ページ'; title='Webhook一覧'; cacheBuster = 'v4' app-screenshot-end]

**New Webhook** をクリックして追加します。各Webhookには URL、1つのコメントイベント（作成、更新、削除のいずれか）、ドメイン、HTTP メソッドがあります:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='URL、イベント、ドメイン、HTTPメソッドのフィールドと「テストペイロード送信」ボタンがある新しいWebhookフォーム'; title='新しいWebhook'; cacheBuster = 'v4' app-screenshot-end]

各Webhookは独立して配信されます。同じイベントを複数のエンドポイントに送信できます。また、**All Domains** にスコープされたWebhookは、同じイベントに対してドメイン固有のWebhookが存在していても、すべてのドメインからのコメントを受け取ります。同一の URL、イベント、ドメインの組み合わせは二度追加できません。

保存する前に **Send Test Payload** をクリックして、エンドポイントが署名付きリクエストを受け入れるか確認してください。詳細は次のセクション「Testing」を参照してください。

一覧からWebhookを編集、無効化、再有効化、削除できます。無効化すると、Webhookが再有効化されるまでキューに入ったイベントは保持されます。削除するとそれらは破棄されます。

WebhookはAPI経由でも作成できます（例: Zapier）。この場合、同じ一覧にソースが **API** として表示されます。API を使用した Webhook の管理については、Managing Webhooks via the API を参照してください。

---