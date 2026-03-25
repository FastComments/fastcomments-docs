req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userIdWS | string | 예 |  |
| startTime | number | 예 |  |
| endTime | number | 예 |  |

## 응답

반환: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## 예제

[inline-code-attrs-start title = 'getEventLog 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-01';
const urlId: string = 'article-2026-03-25';
const userIdWS: string | undefined = undefined; // 선택적 업스트림 값
const startTime: number = Date.parse('2026-03-01T00:00:00Z');
const endTime: number = Date.parse('2026-03-25T23:59:59Z');

const eventLogResponse: GetEventLog200Response = await getEventLog(
  tenantId,
  urlId,
  userIdWS ?? 'ws_user_8b1f',
  startTime,
  endTime
);
[inline-code-end]

---