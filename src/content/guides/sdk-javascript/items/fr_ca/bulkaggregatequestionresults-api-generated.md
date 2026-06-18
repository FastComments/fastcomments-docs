## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Oui |  |
| forceRecalculate | boolean | Non |  |

## Réponse

Renvoie: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResults200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de bulkAggregateQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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