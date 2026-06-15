## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
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

Retorna: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_12345';
const questionId: string = 'q_98765';
const questionIds: string[] = ['q_98765', 'q_12345'];
const urlId: string = 'url_abc123';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 0;
const maxValue: number = 5;
const limit: number = 50;
const result: CombineCommentsWithQuestionResults200Response = await combineCommentsWithQuestionResults(
  tenantId,
  questionId,
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