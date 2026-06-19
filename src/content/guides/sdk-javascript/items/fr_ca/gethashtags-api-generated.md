---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| page | number | Non |  |

## Réponse

Renvoie: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const pageNumber: number = 2;
const responseWithPage: GetHashTagsResponse = await getHashTags(tenantId, pageNumber);
const responseFirstPage: GetHashTagsResponse = await getHashTags(tenantId);
[inline-code-end]

---