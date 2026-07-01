## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| domainToUpdate | string | Oui |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Oui |  |

## Réponse

Renvoie : [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutDomainConfigResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de putDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'tenant-9f8c7b1a';
  const domainToUpdate: string = 'comments.mywebsite.org';
  const updateDomainConfigParams: UpdateDomainConfigParams = {
    enableModeration: true,
    // champ optionnel omis, p. ex., maxCommentLength?: number
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