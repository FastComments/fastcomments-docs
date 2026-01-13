---
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
(async () => {
  const tenantId: string = "acme-corp-7a9f";
  const tagsPage1: GetHashTags200Response = await getHashTags(tenantId);
  const tagsPage2: GetHashTags200Response = await getHashTags(tenantId, 2);
  console.log(tagsPage1, tagsPage2);
})();
[inline-code-end]

---