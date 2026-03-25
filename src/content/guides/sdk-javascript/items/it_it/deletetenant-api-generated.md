## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| sure | string | No |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3d2c";
const id: string = "flag_8392b1a7";
const sure: string = "confirmed";

const responseWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id);
const responseWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
[inline-code-end]