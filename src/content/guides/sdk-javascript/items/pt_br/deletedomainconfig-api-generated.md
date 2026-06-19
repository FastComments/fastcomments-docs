## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| domain | string | Sim |  |

## Resposta

Retorna: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org_5b2f9c3a";
const domain: string = "comments.acme-corp.com";
const result: DeleteDomainConfigResponse = await deleteDomainConfig(tenantId, domain);
[inline-code-end]

---