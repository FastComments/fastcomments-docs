## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Antwort

Gibt zurück: [`GetV2PageReacts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getV2PageReacts Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_82f4b3a9";
const urlId: string = "https://news.site.com/articles/2026/06/15/product-launch";
const response: GetV2PageReacts200Response = await getV2PageReacts(tenantId, urlId);
console.log(response);
[inline-code-end]

---