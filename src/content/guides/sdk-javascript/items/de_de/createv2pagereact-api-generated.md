---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| id | string | Ja |  |
| title | string | Nein |  |

## Antwort

Gibt zurück: [`CreateV2PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV2PageReact200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'createV2PageReact Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments-tenant-72";
const urlId: string = "articles/2026/06/15/product-update";
const id: string = "page-8f3b2a";
const title: string = "Product Update: June 15, 2026";

(async function run(): Promise<void> {
  const response: CreateV2PageReact200Response = await createV2PageReact(tenantId, urlId, id, title);
  console.log(response);
})();
[inline-code-end]

---