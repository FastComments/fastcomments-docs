## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| questionId | string | Hayır |  |
| questionIds | Array<string> | Hayır |  |
| urlId | string | Hayır |  |
| startDate | Date | Hayır |  |
| forceRecalculate | boolean | Hayır |  |
| minValue | number | Hayır |  |
| maxValue | number | Hayır |  |
| limit | number | Hayır |  |

## Yanıt

Döndürür: [`CombineCommentsWithQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResultsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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