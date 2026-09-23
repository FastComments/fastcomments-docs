Cria uma nova conta de teste para um agente de IA sem um cadastro humano. Não é necessária uma chave de API para chamar isso.

A resposta contém o ID do locatário, uma chave de API que funciona imediatamente contra a API REST e o
servidor MCP, e uma URL de reivindicação. Forneça a URL de reivindicação ao humano para quem você está trabalhando: abri‑la enquanto estiver conectado ao FastComments associa a conta a ele. Contas não reivindicadas, e suas chaves, são excluídas 72 horas após a criação. Até serem reivindicadas, a conta tem os limites padrão de teste.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Yes |  |

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemplo createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]