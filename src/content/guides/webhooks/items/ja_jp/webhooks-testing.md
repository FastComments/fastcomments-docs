Webhooks 管理画面には各イベントタイプ（作成、更新、削除）に対して `Send Test Payload` ボタンがあります。作成イベントと更新イベントはダミーの `WebhookComment` オブジェクトを送信しますが、削除イベントをテストする場合は ID のみを含むダミーのリクエストボディが送信されます。

## ペイロードの検証

Webhook 統合をテストする際、受信リクエストに以下のヘッダーが含まれていることを確認してください:

1. **`token`** - あなたの API Secret
2. **`X-FastComments-Timestamp`** - Unix タイムスタンプ（秒）
3. **`X-FastComments-Signature`** - HMAC-SHA256 署名

ペイロードが正当であることを確認するために HMAC 署名の検証を使用してください。

## テスト用ツール

開発中に受信する webhook ペイロードを確認するために、[webhook.site](https://webhook.site) や [ngrok](https://ngrok.com) のようなツールを使用できます。

## イベントタイプ

- **作成イベント**: 新しいコメントが作成されたときにトリガーされます。デフォルトのメソッド: PUT
- **更新イベント**: コメントが編集されたときにトリガーされます。デフォルトのメソッド: PUT
- **削除イベント**: コメントが削除されたときにトリガーされます。デフォルトのメソッド: DELETE

各イベントはリクエストボディに完全なコメントデータを含みます（ペイロードの形式については [Data Structures](/guides/webhooks/webhooks-structures) を参照してください）。

---