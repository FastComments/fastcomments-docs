## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Так |  |
| forceRecalculate | boolean | Ні |  |

## Відповідь

Повертає: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResults200Response.ts)

## Приклад

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_live_7c9d";
const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
  items: [
    { questionId: "q_feedback_rating", includeSubQuestions: false, filters: { locale: "en-US" } }
  ],
  timeBuckets: [
    { granularity: "day", startUtc: "2026-03-01T00:00:00Z", endUtc: "2026-03-15T00:00:00Z" }
  ]
};
const forceRecalculate: boolean = true;
const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
[inline-code-end]