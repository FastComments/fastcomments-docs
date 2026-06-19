## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserBadgeProgressById'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-72b1";
const badgeId: string = "badge-4d9f12";
const result: APIGetUserBadgeProgressResponse = await getUserBadgeProgressById(tenantId, badgeId);
const status: APIStatus | undefined = result?.status;
const progressList: UserBadgeProgress[] | undefined = result?.progress;
const firstProgress: UserBadgeProgress | undefined = progressList?.[0];
[inline-code-end]

---