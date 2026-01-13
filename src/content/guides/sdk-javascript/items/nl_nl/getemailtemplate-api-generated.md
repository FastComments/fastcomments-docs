## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Retourneert: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getEmailTemplate Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-enterprises-123';
  const id: string = 'welcome-email-template-v2';
  const locale: string | undefined = 'en-US'; // voorbeeld van een optionele parameter
  const template: GetEmailTemplate200Response = await getEmailTemplate(tenantId, id);
  console.log(template, locale);
})();
[inline-code-end]

---