In the Webhooks admin there are `Send Test Payload` buttons for each event type (Create, Update, Delete). The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

## ペイロードの検証

Webhook 統合をテストする際、受信リクエストに次のヘッダーが含まれていることを確認してください:

1. **`token`** - あなたの API シークレット
2. **`X-FastComments-Timestamp`** - Unix タイムスタンプ（秒）
3. **`X-FastComments-Signature`** - HMAC-SHA256 署名

HMAC 署名の検証を使用して、ペイロードが正当であることを確認してください。

## テストツール

開発中に受信する webhook ペイロードを検査するために、[webhook.site](https://webhook.site) や [ngrok](https://ngrok.com) などのツールを使用できます。

## Event Types

- **Create Event**: Triggered when a new comment is created. Default method: PUT
- **Update Event**: Triggered when a comment is edited. Default method: PUT
- **Delete Event**: Triggered when a comment is deleted. Default method: DELETE

各イベントはリクエストボディに完全なコメントデータを含みます（ペイロード形式については [データ構造](/guide-webhooks.html#webhooks-structures) を参照してください）。

---