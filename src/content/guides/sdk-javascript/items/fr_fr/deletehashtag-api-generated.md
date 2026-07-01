## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tag | string | Oui |  |
| tenantId | string | Non |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Non |  |

## Réponse

Retourne : [`DeleteHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteHashTagResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tag: string = "announcement";
  const tenantId: string = "tenant_9876";
  const requestBody: DeleteHashTagRequestBody = {
    confirmDeletion: true
  };
  const response: DeleteHashTagResponse = await deleteHashTag(tag, tenantId, requestBody);
  console.log(response);
})();
[inline-code-end]