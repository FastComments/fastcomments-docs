## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 响应

返回：[`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByIdAPIResponse.ts)

## 示例

[inline-code-attrs-start title = 'getSSOUserById 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUser() {
  const tenantId: string = "tenant-987654321";
  const userId: string = "sso-user-abc123";
  const result: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, userId);
  const ssoUser: APISSOUser = result.user;
}
[inline-code-end]

---