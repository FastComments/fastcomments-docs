## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| sso | string | No |  |

## 响应

返回：[`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserTrustFactorResponse.ts)

## 示例

[inline-code-attrs-start title = 'getTrustFactor 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_98765";
  const sso: string = "sso_token_abcde";

  const trustFactor: GetUserTrustFactorResponse = await getTrustFactor(tenantId);
  const trustFactorWithUser: GetUserTrustFactorResponse = await getTrustFactor(tenantId, userId);
  const trustFactorFull: GetUserTrustFactorResponse = await getTrustFactor(tenantId, userId, sso);
}
[inline-code-end]

---