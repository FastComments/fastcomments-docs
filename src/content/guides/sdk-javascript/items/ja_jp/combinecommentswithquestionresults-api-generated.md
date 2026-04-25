## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| questionId | string | いいえ |  |
| questionIds | Array<string> | いいえ |  |
| urlId | string | いいえ |  |
| startDate | Date | いいえ |  |
| forceRecalculate | boolean | いいえ |  |
| minValue | number | いいえ |  |
| maxValue | number | いいえ |  |
| limit | number | いいえ |  |

## レスポンス

戻り値: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## 例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-acme-001';
const questionId: string | undefined = 'q-analytics-42';
const questionIds: string[] | undefined = ['q-analytics-42', 'q-feedback-17'];
const urlId: string | undefined = 'url-987654';
const startDate: Date | undefined = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean | undefined = true;
const minValue: number | undefined = 1;
const maxValue: number | undefined = 5;
const limit: number | undefined = 250;
const result: CombineCommentsWithQuestionResults200Response = await combineCommentsWithQuestionResults(
  tenantId,
  questionId,
  questionIds,
  urlId,
  startDate,
  forceRecalculate,
  minValue,
  maxValue,
  limit
);
[inline-code-end]

---