## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|------|------|
| userId | string | いいえ |  |
| tenantId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTrustFactorResponse.ts)

## 例

[inline-code-attrs-start title = 'getTrustFactor 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDemo(): Promise<void> {
    const trustFull: GetTrustFactorResponse = await getTrustFactor("user_12345", "tenant_abc", "sso_token_987");
    const trustUserOnly: GetTrustFactorResponse = await getTrustFactor("user_12345");
    console.log(trustFull, trustUserOnly);
}
runDemo();
[inline-code-end]

---