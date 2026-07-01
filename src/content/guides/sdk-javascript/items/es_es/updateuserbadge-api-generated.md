## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## Respuesta

Devuelve: [`UpdateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadgeResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function applyBadge() {
    const tenantId: string = "acme-corp-tenant";
    const userId: string = "user-98765";

    const params: UpdateUserBadgeParams = {
        badgeId: "gold-contributor",
        // ejemplo de campo opcional
        expiresAt: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
    };

    const result: UpdateUserBadgeResponse = await updateUserBadge(tenantId, userId, params);
    console.log(result);
}

applyBadge();
[inline-code-end]