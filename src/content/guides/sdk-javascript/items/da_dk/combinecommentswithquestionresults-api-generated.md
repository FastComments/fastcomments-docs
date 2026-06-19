## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| questionId | string | Nej |  |
| questionIds | Array<string> | Nej |  |
| urlId | string | Nej |  |
| startDate | Date | Nej |  |
| forceRecalculate | boolean | Nej |  |
| minValue | number | Nej |  |
| maxValue | number | Nej |  |
| limit | number | Nej |  |

## Svar

Returnerer: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineQuestionResultsWithCommentsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7c9f4b3a';
const questionIds: string[] = ['q-4f8b2a1c', 'q-9d3e7b0f'];
const urlId: string = 'url_93b2c1a7';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 0.2;
const maxValue: number = 0.95;
const limit: number = 100;

const combinedResult: CombineQuestionResultsWithCommentsResponse = await combineCommentsWithQuestionResults({
  tenantId,
  questionIds,
  urlId,
  startDate,
  forceRecalculate,
  minValue,
  maxValue,
  limit
});
[inline-code-end]

---