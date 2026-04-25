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

## Antwoord

Retourneert: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'aggregateQuestionResults Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_001";
const questionIds: string[] = ["q-2026-sales", "q-2026-support"];
const urlId: string = "url_7f2c";
const timeBucket: AggregateTimeBucket = { unit: "week", size: 1 };
const startDate: Date = new Date("2026-01-01T00:00:00Z");
const forceRecalculate: boolean = true;

const result: AggregateQuestionResults200Response = await aggregateQuestionResults(
  tenantId,
  undefined,
  questionIds,
  urlId,
  timeBucket,
  startDate,
  forceRecalculate
);
[inline-code-end]

---