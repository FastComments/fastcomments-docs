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

返却値: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

---