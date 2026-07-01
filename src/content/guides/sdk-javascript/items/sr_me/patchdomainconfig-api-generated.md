## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## Odgovor

Vraća: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primjer patchDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updateDomainConfig() {
  const tenantId: string = "tenant_98765";
  const domainToUpdate: string = "forum.mycompany.com";
  const patchParams: PatchDomainConfigParams = {
    enableComments: true,
    moderationLevel: "strict",
    allowAnonymous: false, // demonstriran opcionalni parametar
  };
  const response: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchParams);
  console.log(response);
}

updateDomainConfig();
[inline-code-end]