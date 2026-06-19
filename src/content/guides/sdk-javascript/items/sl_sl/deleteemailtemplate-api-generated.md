## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrača: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const templateId: string = 'tmpl_3fa85f64-5717-4562-b3fc-2c963f66afa6';
const optionalStatus: APIStatus | undefined = undefined;
const result: APIEmptyResponse = await deleteEmailTemplate(tenantId, templateId);
[inline-code-end]