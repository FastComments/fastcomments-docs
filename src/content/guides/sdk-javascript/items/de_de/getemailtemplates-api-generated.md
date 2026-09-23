## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| skip | number | Nein |  |

## Antwort

Rückgabe: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getEmailTemplates Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEmailTemplates() {
  const tenantId: string = "acme-corp-123";
  const skip: number = 20;
  const templatesWithSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId, skip);
  const templatesWithoutSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId);
}
[inline-code-end]

---