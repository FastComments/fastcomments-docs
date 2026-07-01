## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |

## Svar

Returnerer: [`GetEmailTemplateDefinitionsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse1.ts)

## Eksempel

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const emailTemplateDefs: GetEmailTemplateDefinitionsResponse1 = await getEmailTemplateDefinitions(tenantId);
  console.log(emailTemplateDefs);
})();
[inline-code-end]