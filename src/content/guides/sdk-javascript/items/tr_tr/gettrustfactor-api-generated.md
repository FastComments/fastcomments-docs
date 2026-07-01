## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| userId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Yanıt

Döndürür: [`GetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTrustFactorResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getTrustFactor Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDemo(): Promise<void> {
    const trustFull: GetTrustFactorResponse = await getTrustFactor("user_12345", "tenant_abc", "sso_token_987");
    const trustUserOnly: GetTrustFactorResponse = await getTrustFactor("user_12345");
    console.log(trustFull, trustUserOnly);
}
runDemo();
[inline-code-end]

---