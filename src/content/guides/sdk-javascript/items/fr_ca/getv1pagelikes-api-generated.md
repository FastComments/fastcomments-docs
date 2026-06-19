## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |

## Réponse

Renvoie : [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getV1PageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments-tenant-8a9b3";
const urlId: string = "articles/how-to-optimize-comments-2026-06-19";
const likes: GetV1PageLikes = await getV1PageLikes(tenantId, urlId);
[inline-code-end]

---