---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| sure | string | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'deleteTenant Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_742b9c';
const flagId: string = 'flag_1a2b3c';
const resultWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, flagId);
const sureConfirmation: string = 'confirmed';
const resultWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, flagId, sureConfirmation);
[inline-code-end]

---