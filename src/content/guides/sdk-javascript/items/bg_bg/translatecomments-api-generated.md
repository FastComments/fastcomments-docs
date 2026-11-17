Превежда дадените коментари на езика на уиджета. Налично само когато съответното правило за персонализиране на уиджета има Comment Translation зададено на Automatic или Manual.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| commentTranslationsRequest | CommentTranslationsRequest | Да |  |
| sso | string | Не |  |
| locale | string | Не |  |

## Отговор

Връща: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за translateComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'blog-post-42';
const sso: string = 'JWT_TOKEN_HERE';

const commentTranslationsRequest: CommentTranslationsRequest = {
  commentIds: ['c_98765', 'c_54321'],
  targetLocale: 'es'
};

const locale: string = 'es-ES';

const response: CommentTranslationsResponse = await translateComments(
  tenantId,
  urlId,
  commentTranslationsRequest,
  sso,
  locale
);

console.log(`Translated ${response.translations.length} comments`);
[inline-code-end]