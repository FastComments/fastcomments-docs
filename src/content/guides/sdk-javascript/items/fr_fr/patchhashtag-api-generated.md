## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | Yes |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Réponse

Renvoie : [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Appel sans corps optionnel
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Préparer le corps pour la mise à jour
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]