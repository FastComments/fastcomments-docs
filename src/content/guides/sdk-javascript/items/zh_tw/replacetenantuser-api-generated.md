## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| replaceTenantUserBody | ReplaceTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 回傳

回傳：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'replaceTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-92";
const id: string = "user_7f9b2a";
const replaceTenantUserBody: ReplaceTenantUserBody = {
  email: "maria.garcia@acme-corp.com",
  displayName: "María García",
  role: "moderator",
  externalId: "ext-5271"
};
const updateComments: string = "true";
const result: APIEmptyResponse = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---