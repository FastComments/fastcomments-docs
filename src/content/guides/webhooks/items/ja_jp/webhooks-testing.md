新規および編集用のWebhookページには、`Send Test Payload` ボタンがあり、保存済みかどうかに関わらず、フォームに入力されているURLにリクエストを送信します。CreateイベントとUpdateイベントはダミーのWebhookCommentオブジェクトを送信し、DeleteのテストではIDのみを含むダミーのリクエストボディを送信します。

## ペイロードの検証

Webhook統合をテストする際、受信リクエストに以下のヘッダーが含まれていることを確認してください:

1. **`X-FastComments-Timestamp`** - Unixタイムスタンプ(秒)
2. **`X-FastComments-Signature`** - HMAC-SHA256署名

署名スキームが導入される前に作成されたWebhookも、APIシークレットを含む **`token`** ヘッダーを受け取ります。新しいWebhookでは受け取りません。

ペイロードが本物であることを確認するために、HMAC署名検証を使用してください。

## テストツール

開発中に受信したWebhookペイロードを検査するには、[webhook.site](https://webhook.site) や [ngrok](https://ngrok.com) などのツールを使用できます。

## イベントタイプ

- **Createイベント**: 新しいコメントが作成されたときにトリガーされます。
- **Updateイベント**: コメントが編集されたときにトリガーされます。
- **Deleteイベント**: コメントが削除されたときにトリガーされます。

各Webhookは1つのイベントと1つのHTTPメソッド(POST、PUT、またはDELETE)に紐付けられています。各イベントには、リクエストボディにコメントの完全なデータが含まれます(ペイロード形式については [データ構造](/guide-webhooks.html#webhooks-structures) を参照してください)。