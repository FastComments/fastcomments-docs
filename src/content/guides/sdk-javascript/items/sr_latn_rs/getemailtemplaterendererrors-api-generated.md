## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrors200Response.ts)

## Primer

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-42';
  const id: string = 'tmpl_3fa85f64-5717-4562-b3fc-2c963f66afa6';
  const skip: number = 20;
  const result: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, id, skip);
  console.log(result);
})();
[inline-code-end]

---