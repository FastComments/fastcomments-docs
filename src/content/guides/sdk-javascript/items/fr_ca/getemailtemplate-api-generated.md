## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-enterprises-123';
  const id: string = 'welcome-email-template-v2';
  const locale: string | undefined = 'en-US'; // exemple de paramètre optionnel
  const template: GetEmailTemplate200Response = await getEmailTemplate(tenantId, id);
  console.log(template, locale);
})();
[inline-code-end]

---