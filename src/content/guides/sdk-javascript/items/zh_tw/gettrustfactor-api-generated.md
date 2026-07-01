## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| userId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 回應

返回: [`GetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTrustFactorResponse.ts)

## 範例

[inline-code-attrs-start title = 'getTrustFactor 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDemo(): Promise<void> {
    const trustFull: GetTrustFactorResponse = await getTrustFactor("user_12345", "tenant_abc", "sso_token_987");
    const trustUserOnly: GetTrustFactorResponse = await getTrustFactor("user_12345");
    console.log(trustFull, trustUserOnly);
}
runDemo();
[inline-code-end]

---