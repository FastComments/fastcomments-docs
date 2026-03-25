## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | לא |  |
| userId | string | לא |  |
| startDate | string | לא |  |
| questionId | string | לא |  |
| questionIds | string | לא |  |
| skip | number | לא |  |

## תגובה

מחזיר: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---