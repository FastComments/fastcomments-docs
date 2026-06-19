## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'deletePendingWebhookEvent 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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