## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Response

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo deleteQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function executeDeletion() {
  const tenantId: string = "tenant_12345";
  const configId: string = "config_9876";
  const result: APIEmptyResponse = await deleteQuestionConfig(tenantId, configId);
  console.log(result);
}
executeDeletion();
[inline-code-end]