## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| trustFactor | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetUserTrustFactorResponse.ts)

## 示例

[inline-code-attrs-start title = 'setTrustFactor 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSetTrustFactor() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";
  const trustFactor: string = "high";
  const sso: string = "sso-token-xyz";

  const fullResponse: SetUserTrustFactorResponse = await setTrustFactor(
    tenantId,
    userId,
    trustFactor,
    sso
  );

  const minimalResponse: SetUserTrustFactorResponse = await setTrustFactor(tenantId);
}
[inline-code-end]