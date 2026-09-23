## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Yes |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo updateQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const questionId: string = "question-42";

  const updateBody: UpdateQuestionConfigBody = {
    title: "Revised FAQ Question"
    // isActive, customOptions, etc. são opcionais e omitidos
  };

  const response: APIEmptyResponse = await updateQuestionConfig(tenantId, questionId, updateBody);
  console.log(response);
})();
[inline-code-end]