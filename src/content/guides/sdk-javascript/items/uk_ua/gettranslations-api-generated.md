## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| namespace | string | Так |  |
| component | string | Так |  |
| locale | string | Ні |  |
| useFullTranslationIds | boolean | Ні |  |

## Відповідь

Повертає: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTranslations() {
  const adminDashboard: GetTranslationsResponse = await getTranslations('admin', 'dashboard');
  const userProfileFr: GetTranslationsResponse = await getTranslations('user', 'profile', 'fr-FR', true);
}
[inline-code-end]