## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| urlIdWS | string | Sim |  |
| userIds | string | Sim |  |

## Resposta

Retorna: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const urlIdWS: string = 'wss://comments.fastsite.com/ws/tenant_42';
const userIds: string = 'user_9012,user_3478';
const includePresenceMetadata: boolean | undefined = true; // exemplo de parâmetro opcional
const presenceStatuses: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]

---