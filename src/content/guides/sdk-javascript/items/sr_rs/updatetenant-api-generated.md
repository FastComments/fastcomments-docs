## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateTenantBody | UpdateTenantBody | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4821";
const id: string = "flag_7b9e";
const billingInfo: BillingInfo | undefined = undefined; // опционално — прескочите да бисте задржали тренутне информације о наплати
const updateTenantBody: UpdateTenantBody = {
  name: "Acme News Comments",
  defaultDomain: "comments.acme.com",
  ...(billingInfo ? { billingInfo } : {})
};
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]

---