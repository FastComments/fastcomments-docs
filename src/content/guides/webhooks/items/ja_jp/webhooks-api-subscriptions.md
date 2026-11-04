WebhookはREST APIからも管理できます。Zapierなどの統合がダッシュボードを操作せずにコメントイベントを購読する仕組みで、REST Hooksパターン(購読、イベント受信、購読解除)に従います。

APIによる購読は、ダッシュボードで設定されたWebhookと並んで管理されます。コメントイベントは、Webhookの作成方法にかかわらず、そのドメインに一致するすべてのWebhookに対して、それぞれ個別の配信として送られます。

## 認証

すべてのリクエストには、`x-api-key`ヘッダー(または`API_KEY`クエリパラメーター)にAPIキーを、`tenantId`クエリパラメーターにテナントIDを指定する必要があります。どちらもダッシュボードのAPI Secretページに表示されています。

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
| `url` | はい | 絶対URL(httpまたはhttps)。 |
| `event` | はい | `comment-created`、`comment-updated`、または`comment-deleted`。 |
| `domain` | いいえ | アカウント設定に含まれるドメイン。デフォルトは`*`で、すべてのドメインのイベントを受け取ります。 |
| `method` | いいえ | `POST`(デフォルト)、`PUT`、または`DELETE`。 |

レスポンスには購読情報が含まれます:

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

同じURLを同じイベントとドメインに対して再度購読しようとすると、重複が作成される代わりに既存の購読が返されるため、クライアントは安全にリトライできます。テナントごとに最大50件のAPI購読が可能です。

## 一覧取得

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

テナントのすべてのWebhookを返します。ダッシュボードで管理されているもの(`"source": "dashboard"`)も含まれます。`event`、`domain`、`source`でフィルターできます。

## 購読解除

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

購読を削除すると、その購読のためにキューに残っているイベントも破棄されます。この方法で削除できるのはAPI経由で作成された購読のみです。ダッシュボードのWebhookや、アカウントに存在しないIDを指定した場合は、コード`not-found`とともに`404`が返されます。ダッシュボードのWebhookはWebhooksページで編集してください。

## ペイロードと署名

配信にはダッシュボードのWebhookと同じペイロードが使用され(データ構造を参照)、同じHMACスキームで署名されます(セキュリティとAPIトークンを参照)。API購読では従来の`token`ヘッダーは送られないため、代わりに`X-FastComments-Signature`ヘッダーを検証してください。

## サンプルペイロード

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

アカウントの最新のコメントを、配信時に含まれるのとまったく同じ形式で返します。これにより、最初のイベントが届く前に統合側で実際のサンプルデータを表示できます。`event`は省略可能で、検証のみ行われます。すべてのイベントで同じコメントオブジェクトが配信されるためです。`limit`のデフォルトは3で、1から10まで指定できます。APIクレジットを2消費します。

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

## 410 Goneでの応答

API購読のエンドポイントがHTTP `410 Gone`で応答した場合、FastCommentsはそれを購読解除として扱います。購読はキューに残っているイベントとともに削除され、それ以降の配信は試行されません。ダッシュボードで設定されたWebhookが自動的に削除されることはなく、これらにとって410は通常の失敗です。その他の失敗ステータスはリトライされ、最終的にWebhookが無効化されます。詳細は「仕組みとリトライの処理」を参照してください。

## ダッシュボード

API購読は、ソースが**API**としてWebhooks一覧に表示され、管理者はそこで編集、無効化、再有効化、削除を行えます。