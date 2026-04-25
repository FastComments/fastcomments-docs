## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Так |  |
| forceRecalculate | boolean | Ні |  |

## Відповідь

Повертає: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResults200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад bulkAggregateQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_42";
const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
  questions: [
    { questionId: "q-001", threadId: "thread-1001", questionType: "rating" },
    { questionId: "q-002", threadId: "thread-1002", questionType: "yes_no" }
  ],
  timeRange: { from: "2026-03-01T00:00:00Z", to: "2026-04-01T00:00:00Z" },
  groupBy: ["questionId", "threadId"]
};
const forceRecalculate: boolean = true;
const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
[inline-code-end]

---