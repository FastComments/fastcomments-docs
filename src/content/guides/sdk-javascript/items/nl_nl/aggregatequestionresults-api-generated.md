## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
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
const tenantId: string = 'tenant_acme_98765';
const questionIds: Array<string> = ['q-102', 'q-103'];
const urlId: string = 'url_55b3';
const timeBucket: AggregateTimeBucket = { unit: 'day', size: 7 };
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;

const result: AggregateQuestionResults200Response = await aggregateQuestionResults(
  tenantId,
  undefined, // questionId omitted, using questionIds instead
  questionIds,
  urlId,
  timeBucket,
  startDate,
  forceRecalculate
);
[inline-code-end]