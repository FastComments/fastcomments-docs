## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domainToUpdate | string | Ja |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Ja |  |

## Svar

Returnerer: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutDomainConfigResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'putDomainConfig Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'tenant-9f8c7b1a';
  const domainToUpdate: string = 'comments.mywebsite.org';
  const updateDomainConfigParams: UpdateDomainConfigParams = {
    enableModeration: true,
    // valgfrit felt udeladt, f.eks., maxCommentLength?: number
  };
  const result: PutDomainConfigResponse = await putDomainConfig(
    tenantId,
    domainToUpdate,
    updateDomainConfigParams,
  );
  console.log(result);
}
runExample();
[inline-code-end]

---