## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchQuestionResult(): Promise<void> {
  const tenantId: string = '5a3b8f9c2d1e4a0001aabbcc';
  const questionId: string = 'q-7892-hflu-survey';

  const response: GetQuestionResultResponse = await getQuestionResult(tenantId, questionId);

  const status: APIStatus = response.status;
  const result: QuestionResult | undefined = response.result;
  const meta: MetaItem[] | undefined = response.meta;

  if (result) {
    console.log(`Question ${result.id} status: ${status}`);
  }
}

fetchQuestionResult();
[inline-code-end]