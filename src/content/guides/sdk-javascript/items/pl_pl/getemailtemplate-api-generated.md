## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |

## Odpowiedź

Zwraca: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-enterprises-123';
  const id: string = 'welcome-email-template-v2';
  const locale: string | undefined = 'en-US'; // przykład opcjonalnego parametru
  const template: GetEmailTemplate200Response = await getEmailTemplate(tenantId, id);
  console.log(template, locale);
})();
[inline-code-end]

---