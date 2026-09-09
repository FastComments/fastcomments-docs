Webhooks は REST API でも管理できます。これは Zapier などの統合がダッシュボードに触れずにコメントイベントを購読する方法で、REST Hooks パターンに従います: 購読、イベント受信、購読解除。

API サブスクリプションはダッシュボードで設定された Webhooks と並行して存在します。コメントイベントはそのドメインのダッシュボード Webhook と、条件に合致するすべての API サブスクリプションにそれぞれ別々に配信されます。イベントあたりの購読者数に制限はありません。

## 認証

すべてのリクエストには `x-api-key` ヘッダー（または `API_KEY` クエリパラメータ）に API キーを、`tenantId` クエリパラメータにテナント ID を含める必要があります。両方ともダッシュボードの API シークレットページに表示されています。

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

| フィールド | 必須 | 説明 |
|-------|----------|-------------|
| `url` | Yes | 絶対的な http または https の URL。 |
| `event` | Yes | `comment-created`、`comment-updated`、または `comment-deleted`。 |
| `domain` | No | アカウント設定からのドメイン。デフォルトは `*` で、すべてのドメインのイベントを受信します。 |
| `method` | No | `POST`（デフォルト）、`PUT`、または `DELETE`。 |

レスポンスにはサブスクリプションが含まれます:

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

同じ URL を同じイベントとドメインで再度購読すると、重複作成されず既存のサブスクリプションが返されるため、クライアントは安全に再試行できます。各テナントは最大 50 件の API サブスクリプションを持つことができます。

## 一覧取得

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

テナントのすべての Webhook が返されます。ダッシュボードで管理されているもの（`"source": "dashboard"`）も含まれます。`event`、`domain`、または `source` でフィルタリングできます。

## 購読解除

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

サブスクリプションを削除すると、キューに残っているイベントも破棄されます。この方法で削除できるのは API 経由で作成されたサブスクリプションのみです。ダッシュボードの Webhook は Webhooks ページで編集します。

## ペイロードと署名

配信はダッシュボード Webhook と同じペイロードを使用し（Data Structures 参照）、同じ HMAC スキームで署名されます（Security & API Tokens 参照）。API サブスクリプションはレガシーな `token` ヘッダーを受け取らないため、代わりに `X-FastComments-Signature` ヘッダーを検証してください。

## 410 Gone での応答

API サブスクリプションのエンドポイントが HTTP `410 Gone` を返すと、FastComments はそれを購読解除とみなし、サブスクリプションとキューに残っているイベントを削除し、以降の配信は行いません。ダッシュボードで設定された Webhook は自動的に削除されず、410 は単なる失敗として扱われます。他の失敗ステータスは再試行され、最終的に Webhook が無効化されます（How it Works & Handling Retries 参照）。

## ダッシュボード

API サブスクリプションは作成されたドメインごとに Webhooks ページに一覧表示され、管理者はそれらを無効化、再有効化、または削除できます。