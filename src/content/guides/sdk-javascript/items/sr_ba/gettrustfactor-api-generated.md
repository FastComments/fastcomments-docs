## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| userId | string | Ne |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`GetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTrustFactorResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getTrustFactor'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDemo(): Promise<void> {
    const trustFull: GetTrustFactorResponse = await getTrustFactor("user_12345", "tenant_abc", "sso_token_987");
    const trustUserOnly: GetTrustFactorResponse = await getTrustFactor("user_12345");
    console.log(trustFull, trustUserOnly);
}
runDemo();
[inline-code-end]