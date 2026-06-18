## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| questionId | string | Nej |  |
| questionIds | Array<string> | Nej |  |
| urlId | string | Nej |  |
| timeBucket | AggregateTimeBucket | Nej |  |
| startDate | Date | Nej |  |
| forceRecalculate | boolean | Nej |  |

## Svar

Returnerer: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'aggregateQuestionResults Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f2b3c';
const questionIds: Array<string> = ['q-7a1b2c', 'q-8d3e4f'];
const urlId: string = 'url_9f8e7d';
const startDate: Date = new Date('2025-01-01T00:00:00Z');
const result: AggregateQuestionResults200Response = await aggregateQuestionResults(tenantId, undefined, questionIds, urlId, undefined, startDate, true);
[inline-code-end]

---