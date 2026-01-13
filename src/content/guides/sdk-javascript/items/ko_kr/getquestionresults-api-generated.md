## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 아니요 |  |
| userId | string | 아니요 |  |
| startDate | string | 아니요 |  |
| questionId | string | 아니요 |  |
| questionIds | string | 아니요 |  |
| skip | number | 아니요 |  |

## 응답

반환: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## 예제

[inline-code-attrs-start title = 'getQuestionResults 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const urlId: string = 'news/2025/fastcomments-release';
const userId: string = 'user_8b7f3c';
const startDate: string = '2025-01-01T00:00:00Z';
const questionIds: string = 'q123,q124';
const skip: number = 20;
const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, undefined, questionIds, skip);
[inline-code-end]

---