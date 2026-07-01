## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| userId | string | Не |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTrustFactorResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getTrustFactor'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDemo(): Promise<void> {
    const trustFull: GetTrustFactorResponse = await getTrustFactor("user_12345", "tenant_abc", "sso_token_987");
    const trustUserOnly: GetTrustFactorResponse = await getTrustFactor("user_12345");
    console.log(trustFull, trustUserOnly);
}
runDemo();
[inline-code-end]