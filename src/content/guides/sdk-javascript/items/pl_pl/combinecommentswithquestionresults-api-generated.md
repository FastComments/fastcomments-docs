## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| questionId | string | Nie |  |
| questionIds | Array<string> | Nie |  |
| urlId | string | Nie |  |
| startDate | Date | Nie |  |
| forceRecalculate | boolean | Nie |  |
| minValue | number | Nie |  |
| maxValue | number | Nie |  |
| limit | number | Nie |  |

## Odpowiedź

Zwraca: [`CombineCommentsWithQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResultsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---