## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Tak |  |

## Odpowiedź

Zwraca: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9f4a2b';
  const createEmailTemplateBody: CreateEmailTemplateBody = {
    name: 'Weekly Digest',
    subject: 'Your weekly discussion highlights',
    html: '<!doctype html><body><h1>Hello \{{user.name}}</h1><p>Top comments this week...</p></body>',
    fromAddress: 'no-reply@fastcomments-example.com',
    replyTo: 'moderation@fastcomments-example.com',
    isDefault: false
  };
  const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
  console.log(result);
})();
[inline-code-end]

---