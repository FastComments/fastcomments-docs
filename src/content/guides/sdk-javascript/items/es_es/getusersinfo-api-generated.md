---
Información masiva de usuarios para un tenant. Dado userIds, devuelve la información de visualización de User / SSOUser.
Utilizado por el widget de comentarios para enriquecer usuarios que acaban de aparecer mediante un evento de presencia.
Sin contexto de página: la privacidad se aplica de forma uniforme (los perfiles privados se enmascaran).

## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| ids | string | Sí |  |

## Respuesta

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo solo requiere tenantId e ids; los parámetros opcionales no son aplicables aquí.
[inline-code-end]

---