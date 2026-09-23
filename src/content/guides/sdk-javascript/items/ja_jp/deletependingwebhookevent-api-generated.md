## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'deletePendingWebhookEvent の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletion() {
  const tenantId: string = "tenant-42f7b9c2";
  const eventId: string = "webhook-event-8a7d6c5b";
  const result: APIEmptyResponse = await deletePendingWebhookEvent(tenantId, eventId);
  console.log(result);
}
[inline-code-end]

---