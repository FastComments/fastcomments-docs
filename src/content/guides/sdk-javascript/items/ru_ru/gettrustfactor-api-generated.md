## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| userId | string | Нет |  |
| tenantId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`GetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTrustFactorResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getTrustFactor'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDemo(): Promise<void> {
    const trustFull: GetTrustFactorResponse = await getTrustFactor("user_12345", "tenant_abc", "sso_token_987");
    const trustUserOnly: GetTrustFactorResponse = await getTrustFactor("user_12345");
    console.log(trustFull, trustUserOnly);
}
runDemo();
[inline-code-end]