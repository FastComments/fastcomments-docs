## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nein |  |

## Antwort

Gibt zurück: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getHashTags Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4b2c3a';
const tagsFirstPage: GetHashTags200Response = await getHashTags(tenantId);
const tagsSecondPage: GetHashTags200Response = await getHashTags(tenantId, 2);
console.log(tagsFirstPage, tagsSecondPage);
[inline-code-end]

---