## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| questionId | string | 否 |  |
| questionIds | Array<string> | 否 |  |
| urlId | string | 否 |  |
| timeBucket | AggregateTimeBucket | 否 |  |
| startDate | Date | 否 |  |
| forceRecalculate | boolean | 否 |  |

## 回應

回傳: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## 範例

[inline-code-attrs-start title = 'aggregateQuestionResults 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_98765';
const questionIds: Array<string> = ['q-102', 'q-103'];
const urlId: string = 'url_55b3';
const timeBucket: AggregateTimeBucket = { unit: 'day', size: 7 };
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;

const result: AggregateQuestionResults200Response = await aggregateQuestionResults(
  tenantId,
  undefined, // 已省略 questionId，改用 questionIds
  questionIds,
  urlId,
  timeBucket,
  startDate,
  forceRecalculate
);
[inline-code-end]

---