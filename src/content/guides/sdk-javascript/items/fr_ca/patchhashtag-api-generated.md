## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tag | string | Oui |  |
| tenantId | string | Non |  |
| updateHashTagBody | UpdateHashTagBody | Non |  |

## Réponse

Renvoie: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = 'release-2026';
const tenantId: string = 'tenant_42';
const updateHashTagBody: UpdateHashTagBody = {
  displayName: 'Release 2026',
  description: 'Discussions and notes for the 2026 product release',
  isActive: true
};
const result: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---