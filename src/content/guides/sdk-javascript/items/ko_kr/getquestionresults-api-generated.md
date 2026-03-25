## 매개변수

| 이름 | 형식 | 필수 | 설명 |
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
const tenantId: string = "tenant_acme_001";
const urlId: string = "articles/product-launch-2026";
const userId: string = "user_2048";
const startDate: string = "2026-03-01T00:00:00Z";
const questionId: string | undefined = undefined;
const questionIds: string | undefined = "q_101,q_102";
const skip: number | undefined = 0;

const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, questionId, questionIds, skip);
[inline-code-end]