---
## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Primjer

[inline-code-attrs-start title = 'getEmailTemplate Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-enterprises-123';
  const id: string = 'welcome-email-template-v2';
  const locale: string | undefined = 'en-US'; // primjer opcionog parametra
  const template: GetEmailTemplate200Response = await getEmailTemplate(tenantId, id);
  console.log(template, locale);
})();
[inline-code-end]

---