---
## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Respons

Geeft terug: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getEmailTemplates Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-marketing-tenant-001";
  const templatesDefault: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
  const templatesPaged: GetEmailTemplates200Response = await getEmailTemplates(tenantId, 25);
  console.log(templatesDefault, templatesPaged);
}
run();
[inline-code-end]

---