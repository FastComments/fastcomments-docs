## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| page | number | Non |  |

## Réponse

Renvoie : [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4b2c3a';
const tagsFirstPage: GetHashTags200Response = await getHashTags(tenantId);
const tagsSecondPage: GetHashTags200Response = await getHashTags(tenantId, 2);
console.log(tagsFirstPage, tagsSecondPage);
[inline-code-end]

---