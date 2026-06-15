## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Sí |  |
| forceRecalculate | boolean | No |  |

## Respuesta

Devuelve: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResults200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de bulkAggregateQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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