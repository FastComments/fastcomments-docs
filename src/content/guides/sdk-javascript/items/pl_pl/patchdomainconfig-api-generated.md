## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| domainToUpdate | string | Tak |  |
| patchDomainConfigParams | PatchDomainConfigParams | Tak |  |

## Odpowiedź

Zwraca: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład patchDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updateDomainConfig() {
  const tenantId: string = "tenant_98765";
  const domainToUpdate: string = "forum.mycompany.com";
  const patchParams: PatchDomainConfigParams = {
    enableComments: true,
    moderationLevel: "strict",
    allowAnonymous: false, // zaprezentowano opcjonalny parametr
  };
  const response: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchParams);
  console.log(response);
}

updateDomainConfig();
[inline-code-end]