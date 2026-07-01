## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| domainToUpdate | string | Sim |  |
| patchDomainConfigParams | PatchDomainConfigParams | Sim |  |

## Response

Retorna: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'patchDomainConfig Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updateDomainConfig() {
  const tenantId: string = "tenant_98765";
  const domainToUpdate: string = "forum.mycompany.com";
  const patchParams: PatchDomainConfigParams = {
    enableComments: true,
    moderationLevel: "strict",
    allowAnonymous: false, // parâmetro opcional demonstrado
  };
  const response: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchParams);
  console.log(response);
}

updateDomainConfig();
[inline-code-end]

---