## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantBody | UpdateTenantBody | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'updateTenant 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4821";
const id: string = "flag_7b9e";
const billingInfo: BillingInfo | undefined = undefined; // 選填，省略以保留目前的帳單資訊
const updateTenantBody: UpdateTenantBody = {
  name: "Acme News Comments",
  defaultDomain: "comments.acme.com",
  ...(billingInfo ? { billingInfo } : {})
};
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]