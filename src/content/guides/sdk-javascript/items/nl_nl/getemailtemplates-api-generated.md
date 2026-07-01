## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetEmailTemplatesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getEmailTemplates Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  // Aanroep zonder optionele 'skip'
  const templates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId);

  // Aanroep met optionele 'skip' parameter
  const pagedTemplates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId, 20);
})();
[inline-code-end]

---