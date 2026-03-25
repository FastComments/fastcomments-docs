## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| sure | string | Não |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3d2c";
const id: string = "flag_8392b1a7";
const sure: string = "confirmed";

const responseWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id);
const responseWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
[inline-code-end]

---