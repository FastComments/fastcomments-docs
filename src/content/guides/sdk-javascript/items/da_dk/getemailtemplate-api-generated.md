## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getEmailTemplate Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTemplate() {
  const tenantId: string = "acme-corp-123";
  const templateId: string = "welcome-email-456";
  const response: GetEmailTemplateResponse = await getEmailTemplate(tenantId, templateId);
  const status: APIStatus | undefined = response.status;
  const customTemplate: CustomEmailTemplate | undefined = response.template;
}
[inline-code-end]

---