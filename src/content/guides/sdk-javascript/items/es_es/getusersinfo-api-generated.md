---
Información de usuarios en bloque para un inquilino. Dado userIds, devuelve la información de visualización de User / SSOUser.  
Utilizado por el widget de comentarios para enriquecer a los usuarios que acaban de aparecer mediante un evento de presencia.  
Sin contexto de página: la privacidad se aplica de manera uniforme (los perfiles privados se enmascaran).

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| ids | string | Sí |  |

## Respuesta

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]

---