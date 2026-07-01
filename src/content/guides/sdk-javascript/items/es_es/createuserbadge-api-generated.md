## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createUserBadgeParams | CreateUserBadgeParams | Sí |  |

## Respuesta

Devuelve: [`CreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadgeResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "company-42";

const badgeParams: CreateUserBadgeParams = {
  name: "Community Champion",
  iconUrl: "https://assets.example.com/badges/champion.png",
  // la descripción es opcional y se omite aquí
};

const result: CreateUserBadgeResponse = await createUserBadge(tenantId, badgeParams);
[inline-code-end]