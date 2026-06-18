req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userIdWS | string | 예 |  |
| startTime | number | 예 |  |
| endTime | number | 아니요 |  |

## 응답

반환: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## 예제

[inline-code-attrs-start title = 'getGlobalEventLog 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_639b7f12";
const urlId: string = "https://www.news-site.com/articles/2026/06/15/important-update-987";
const userIdWS: string = "user_ws_42b7";
const startTime: number = new Date("2026-06-14T00:00:00Z").getTime();
const endTime: number = Date.now();

const eventLog: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]