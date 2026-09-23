## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| id | string | Ja |  |
| title | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Beispiel

[inline-code-attrs-start title = 'createV2PageReact Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "acme-corp";
  const urlId: string = "product-page-123";
  const pageId: string = "page-456";
  const pageTitle: string = "Product Overview";
  const ssoToken: string = "jwt-token-abc123";

  const pageFull: CreateV1PageReact = await createV2PageReact(tenantId, urlId, pageId, pageTitle, ssoToken);
  const pageTitleOnly: CreateV1PageReact = await createV2PageReact(tenantId, urlId, pageId, pageTitle);
  const pageMinimal: CreateV1PageReact = await createV2PageReact(tenantId, urlId, pageId);
}
run();
[inline-code-end]

---