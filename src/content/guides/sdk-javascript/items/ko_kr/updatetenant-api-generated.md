## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantBody | UpdateTenantBody | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'updateTenant 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_01H4ZQ7KABCD";
const id: string = "site_9f8e7d6c";
const apiDomainConfiguration: APIDomainConfiguration = {
  primaryDomain: "comments.acme.com",
  allowSubdomains: true
};
const billingInfo: BillingInfo = {
  planName: "Business",
  billingContactEmail: "billing@acme.com",
  seats: 25
};
const updateTenantBody: UpdateTenantBody = {
  displayName: "Acme Corporation Comments",
  apiDomainConfiguration,
  billingInfo, // 선택적 매개변수 예시
  enableModeration: true
};
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]

---