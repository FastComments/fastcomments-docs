---
Webhooks は REST API を通じても管理できます。これは、Zapier のような統合がダッシュボードに触れずにコメントイベントを購読する方法で、REST Hooks パターン（購読、イベント受信、購読解除）に従います。

API サブスクリプションは、ダッシュボードで設定された Webhooks と共存します。コメントイベントは、ドメインが一致するすべての Webhook に対して個別に配信され、Webhook の作成方法に関係なく配信されます。

## Authentication

すべてのリクエストは、`x-api-key` ヘッダー（または `API_KEY` クエリパラメータ）に API キーを、`tenantId` クエリパラメータにテナント ID を含める必要があります。これらはダッシュボードの API Secret ページに表示されています。

## Subscribe

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
| `event` | Yes | `comment-created`、`comment-updated`、または `comment-deleted`。 |
| `domain` | No | アカウント設定からのドメイン。デフォルトは `*` で、すべてのドメインのイベントを受信します。 |
| `method` | No | `POST`（デフォルト）、`PUT`、または DELETE。 |

レスポンスにはサブスクリプションが含まれます：

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

## List

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

テナントのすべての Webhook を返します。ダッシュボードで管理されているもの（`"source": "dashboard"`）も含まれます。`event`、`domain`、または `source` でフィルタリングできます。

## Unsubscribe

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

サブスクリプションを削除すると、キューに残っているイベントも破棄されます。この方法で削除できるのは API 経由で作成されたサブスクリプションのみです。ダッシュボードの Webhook や、アカウントに存在しない ID は `404` とコード `not-found` を返します。ダッシュボードの Webhook は Webhooks ページで編集します。

## Payloads and signing

配信はダッシュボードの Webhook と同じペイロードを使用し（Data Structures 参照）、同じ HMAC 方式で署名されます（Security & API Tokens 参照）。API サブスクリプションはレガシーの `token` ヘッダーを受け取らないため、代わりに `X-FastComments-Signature` ヘッダーを検証してください。

## Sample payloads

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

## Responding with 410 Gone

API サブスクリプションのエンドポイントが HTTP `410 Gone` を返した場合、FastComments はそれを購読解除とみなし、サブスクリプションとキューに残っているイベントが削除され、以降の配信は行われません。ダッシュボードで設定された Webhook は自動的に削除されることはなく、410 は通常の失敗として扱われます。その他の失敗ステータスは再試行され、最終的に Webhook が無効化されます（How it Works & Handling Retries 参照）。

## Dashboard

API サブスクリプションは Webhooks リストに **API** ソースとして表示され、管理者はそれらを編集、無効化、再有効化、または削除できます。

---