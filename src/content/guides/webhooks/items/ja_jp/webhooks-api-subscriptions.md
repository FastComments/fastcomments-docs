Webhooks は REST API を通じても管理できます。これは Zapier などの統合がダッシュボードに触れずにコメントイベントを購読する方法で、REST Hooks パターン（購読、イベント受信、購読解除）に従います。

API サブスクリプションはダッシュボードで設定された Webhooks と共存します。コメントイベントは、ドメインが一致するすべての Webhook に個別の配信として送信され、Webhook の作成方法に関係なく配信されます。

## 認証

すべてのリクエストは `x-api-key` ヘッダー（または `API_KEY` クエリパラメータ）に API キーを、`tenantId` クエリパラメータにテナント ID を含める必要があります。これらはダッシュボードの API シークレットページに表示されています。

## 購読

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Field | Required | Description |
|-------|----------|-------------|
| `url` | Yes | 絶対的な http または https の URL。 |
| `event` | Yes | `comment-created`, `comment-updated` or `comment-deleted`. |
| `domain` | No | アカウント設定からのドメイン。デフォルトは `*` で、すべてのドメインのイベントを受信します。 |
| `method` | No | `POST` (default), `PUT` or `DELETE`. |

レスポンスにはサブスクリプションが含まれます。

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

同じ URL を同じイベントとドメインで再度購読すると、重複を作成せずに既存のサブスクリプションが返されるため、クライアントは安全に再試行できます。各テナントは最大 50 件の API サブスクリプションを持つことができます。

## 一覧

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

テナントのすべての Webhook を返します。ダッシュボードで管理されているもの（`"source": "dashboard"`）も含まれます。`event`、`domain`、または `source` でフィルタできます。

## 購読解除

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

サブスクリプションを削除すると、キューに残っているイベントも破棄されます。この方法で削除できるのは API 経由で作成されたサブスクリプションのみです。ダッシュボードの Webhook は Webhooks ページで編集します。

## ペイロードと署名

配信はダッシュボードの Webhook と同じペイロードを使用し（Data Structures を参照）、同じ HMAC 方式で署名されます（Security & API Tokens を参照）。API サブスクリプションはレガシーな `token` ヘッダーを受け取らないため、代わりに `X-FastComments-Signature` ヘッダーを検証してください。

## サンプルペイロード

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

配信が保持する形そのままで、アカウントの最新コメントを返すため、統合は最初のイベントが到着する前に実際のサンプルデータを表示できます。`event` はオプションで、すべてのイベントが同じコメントオブジェクトを配信するため検証のみ行われます。`limit` のデフォルトは 3 で、1 から 10 の範囲で指定可能です。2 API クレジットが消費されます。

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## 410 Gone に応答する

API サブスクリプションのエンドポイントが HTTP `410 Gone` を返した場合、FastComments はそれを購読解除とみなし、サブスクリプションとキューに残っているイベントを削除し、以降の配信は行いません。ダッシュボードで設定された Webhook は自動的に削除されることはなく、410 は通常の失敗として扱われます。その他の失敗ステータスは再試行され、最終的に Webhook が無効化されます（How it Works & Handling Retries を参照）。

## ダッシュボード

API サブスクリプションは Webhooks リストに **API** ソースとして表示され、管理者はそれらを編集、無効化、再有効化、または削除できます。