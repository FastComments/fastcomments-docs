## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_9876";
const configsWithoutSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const configsWithSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, 20);
[inline-code-end]

---