The new and edit webhook pages have a `Send Test Payload` button that sends a request to the URL currently in the form, whether or not it has been saved. The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

## ペイロードの検証

When testing your webhook integration, verify the incoming requests include the following headers:

1. **`X-FastComments-Timestamp`** - Unix タイムスタンプ（秒）
2. **`X-FastComments-Signature`** - HMAC-SHA256 署名

Webhooks created before the signature scheme was introduced also receive a **`token`** header containing your API Secret. New webhooks do not.

Use the HMAC signature verification to ensure payloads are authentic.

## テストツール

You can use tools like [webhook.site](https://webhook.site) or [ngrok](https://ngrok.com) to inspect incoming webhook payloads during development.

## イベントタイプ

- **Create Event**: 新しいコメントが作成されたときにトリガーされます。
- **Update Event**: コメントが編集されたときにトリガーされます。
- **Delete Event**: コメントが削除されたときにトリガーされます。

Each webhook is tied to one event and one HTTP method (POST, PUT or DELETE). Each event includes the full comment data in the request body (see [Data Structures](/guide-webhooks.html#webhooks-structures) for the payload format).

---