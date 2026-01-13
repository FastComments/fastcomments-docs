## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | いいえ |  |
| userId | string | いいえ |  |
| startDate | string | いいえ |  |
| questionId | string | いいえ |  |
| questionIds | string | いいえ |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## 例

[inline-code-attrs-start title = 'getQuestionResults の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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