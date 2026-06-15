## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Да |  |
| forceRecalculate | boolean | Не |  |

## Одговор

Враћа: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResults200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример bulkAggregateQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_0012";
const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
  items: [
    {
      questionId: "quality_score",
      startTime: "2026-06-01T00:00:00Z",
      endTime: "2026-06-14T00:00:00Z",
      timeBucket: "day",
      dimensions: ["threadId"]
    }
  ],
  includeTotals: true
};
const forceRecalculate: boolean = false;
const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
[inline-code-end]

---