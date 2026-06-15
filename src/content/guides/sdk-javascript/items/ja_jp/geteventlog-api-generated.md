req
tenantId
urlId
userIdWS

## パラメーター

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| userIdWS | string | はい |  |
| startTime | number | はい |  |
| endTime | number | いいえ |  |

## レスポンス

返却: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## 例

[inline-code-attrs-start title = 'getEventLog の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3a2b';
const urlId: string = 'news/2026/06/fastcomments-release';
const userIdWS: string = 'ws_user_48291';
const startTime: number = Date.now() - 86_400_000;
const endTime: number = Date.now();
const result: GetEventLog200Response = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]

---