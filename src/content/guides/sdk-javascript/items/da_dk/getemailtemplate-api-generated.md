## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getEmailTemplate Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-marketing-042";
const templateId: string = "tpl_welcome_2026";
const result: GetEmailTemplate200Response = await getEmailTemplate(tenantId, templateId);
const template: CustomEmailTemplate | undefined = result.template;
const subject: string | undefined = template?.subject;
const customParams: CustomConfigParameters | undefined = template?.customConfigParameters;
[inline-code-end]

---