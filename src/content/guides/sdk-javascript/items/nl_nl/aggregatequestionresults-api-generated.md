## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| questionId | string | Nee |  |
| questionIds | Array<string> | Nee |  |
| urlId | string | Nee |  |
| timeBucket | AggregateTimeBucket | Nee |  |
| startDate | Date | Nee |  |
| forceRecalculate | boolean | Nee |  |

## Respons

Retourneert: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'aggregateQuestionResults Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f2b3c';
const questionIds: Array<string> = ['q-7a1b2c', 'q-8d3e4f'];
const urlId: string = 'url_9f8e7d';
const startDate: Date = new Date('2025-01-01T00:00:00Z');
const result: AggregateQuestionResults200Response = await aggregateQuestionResults(tenantId, undefined, questionIds, urlId, undefined, startDate, true);
[inline-code-end]

---