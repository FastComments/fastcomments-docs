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

Returnerer: [`CombineCommentsWithQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResultsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const questionIds: string[] = ["question_1", "question_2"];
const urlId: string = "article-9876";
const startDate: Date = new Date("2023-01-01T00:00:00Z");
const forceRecalculate: boolean = false;
const minValue: number = 1;
const maxValue: number = 5;
const limit: number = 50;

const result: CombineCommentsWithQuestionResultsResponse = await combineCommentsWithQuestionResults(
  tenantId,
  undefined,
  questionIds,
  urlId,
  startDate,
  forceRecalculate,
  minValue,
  maxValue,
  limit
);
[inline-code-end]