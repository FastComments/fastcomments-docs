## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tag | string | Oui |  |
| tenantId | string | Non |  |
| updateHashTagBody | UpdateHashTagBody | Non |  |

## Réponse

Renvoie: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "feature-request";
const tenantId: string = "tenant_8f7a3b2c";
const updateHashTagBody: UpdateHashTagBody = {
  displayName: "Feature Request",
  description: "Use this tag for requests to add new features to the product",
  enabled: true
};
const result: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---