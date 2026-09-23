req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userIdWS | string | 예 |  |
| startTime | number | 예 |  |
| endTime | number | 아니오 |  |

## 응답

반환: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## 예시

[inline-code-attrs-start title = 'getEventLog 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "url_9876";
const userIdWS: string = "user_abcde";
const startTime: number = Date.now() - 86_400_000; // 24시간 전
const endTime: number = Date.now();

const fullLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const partialLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]

---