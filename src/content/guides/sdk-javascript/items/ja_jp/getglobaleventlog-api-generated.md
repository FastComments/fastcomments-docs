リクエスト
tenantId
urlId
userIdWS

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| userIdWS | string | はい |  |
| startTime | number | はい |  |
| endTime | number | いいえ |  |

## Response

戻り値: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## 例

[inline-code-attrs-start title = 'getGlobalEventLog の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f7b2a9c';
const urlId: string = 'article-87c1a2b';
const userIdWS: string = 'ws-1a2b3c4d';
const startTime: number = Date.now() - 60 * 60 * 1000; // 1時間前
const endTime: number = Date.now();

const responseWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const responseWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]