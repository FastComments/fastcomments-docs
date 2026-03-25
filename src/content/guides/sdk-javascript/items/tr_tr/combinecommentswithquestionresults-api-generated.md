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

Döndürür: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## Örnek

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3a2';
const questionIds: string[] = ['question-42', 'question-43'];
const urlId: string = 'url_5d7b';
const startDate: Date = new Date('2026-02-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 1;
const maxValue: number = 5;
const limit: number = 50;
const result: CombineCommentsWithQuestionResults200Response = await combineCommentsWithQuestionResults(
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