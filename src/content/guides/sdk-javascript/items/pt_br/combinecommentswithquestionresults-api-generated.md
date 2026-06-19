## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| questionId | string | Não |  |
| questionIds | Array<string> | Não |  |
| urlId | string | Não |  |
| startDate | Date | Não |  |
| forceRecalculate | boolean | Não |  |
| minValue | number | Não |  |
| maxValue | number | Não |  |
| limit | number | Não |  |

## Resposta

Retorna: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineQuestionResultsWithCommentsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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