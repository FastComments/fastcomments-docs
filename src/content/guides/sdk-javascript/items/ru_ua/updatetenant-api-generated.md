## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantBody | UpdateTenantBody | Yes |  |

## Ответ

Возвращает: [`UpdateTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateTenantResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c8f9e3d2-4b6a-11ee-8c99-0242ac130003";
const id: string = "tenant-config-01";

const updateBody: UpdateTenantBody = {
  domain: "mytenant.fastcomments.io",
  branding: {
    logoUrl: "https://cdn.mytenant.com/assets/logo.png"
  },
  description: "Branding update for Q3"
};

const response: UpdateTenantResponse = await updateTenant(tenantId, id, updateBody);
console.log(response);
[inline-code-end]