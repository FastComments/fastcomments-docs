## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| questionId | string | いいえ |  |
| questionIds | Array<string> | いいえ |  |
| urlId | string | いいえ |  |
| timeBucket | AggregateTimeBucket | いいえ |  |
| startDate | Date | いいえ |  |
| forceRecalculate | boolean | いいえ |  |

## レスポンス

戻り値: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## 例

[inline-code-attrs-start title = 'aggregateQuestionResults の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f2b3c';
const questionIds: Array<string> = ['q-7a1b2c', 'q-8d3e4f'];
const urlId: string = 'url_9f8e7d';
const startDate: Date = new Date('2025-01-01T00:00:00Z');
const result: AggregateQuestionResults200Response = await aggregateQuestionResults(tenantId, undefined, questionIds, urlId, undefined, startDate, true);
[inline-code-end]

---