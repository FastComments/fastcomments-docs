## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| replaceTenantUserBody | ReplaceTenantUserBody | はい |  |
| updateComments | string | いいえ |  |

## 応答

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'replaceTenantUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const userId: string = "user-987654321";
const replaceBody: ReplaceTenantUserBody = {
  email: "newuser@example.com",
  name: "New User",
  role: "admin"
};

const response: APIEmptyResponse = await replaceTenantUser(
  tenantId,
  userId,
  replaceBody,
  "Migrated user to new tenant"
);
[inline-code-end]

---