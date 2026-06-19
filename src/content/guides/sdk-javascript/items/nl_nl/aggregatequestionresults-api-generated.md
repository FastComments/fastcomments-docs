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

## Antwoord

Retourneert: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResultsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'aggregateQuestionResults Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a4f7c2b';
const questionIds: string[] = ['q_101', 'q_102', 'q_103'];
const urlId: string = 'url_3f9b12';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const result: AggregateQuestionResultsResponse = await aggregateQuestionResults(tenantId, undefined, questionIds, urlId, undefined, startDate, forceRecalculate);
[inline-code-end]