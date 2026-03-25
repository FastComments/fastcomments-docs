## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateTenantBody | UpdateTenantBody | Да |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4821";
const id: string = "flag_7b9e";
const billingInfo: BillingInfo | undefined = undefined; // необязательно, опустите, чтобы сохранить текущую информацию о выставлении счетов
const updateTenantBody: UpdateTenantBody = {
  name: "Acme News Comments",
  defaultDomain: "comments.acme.com",
  ...(billingInfo ? { billingInfo } : {})
};
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]

---