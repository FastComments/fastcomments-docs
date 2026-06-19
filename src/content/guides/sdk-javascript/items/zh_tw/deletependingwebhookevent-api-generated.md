## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'deletePendingWebhookEvent 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removePendingWebhookEvent(tenantId?: string): Promise<APIEmptyResponse | undefined> {
  if (!tenantId) return;
  const tenant: string = tenantId;
  const eventId: string = 'evt_7f2c1a9b';
  const response: APIEmptyResponse = await deletePendingWebhookEvent(tenant, eventId);
  return response;
}
[inline-code-end]

---