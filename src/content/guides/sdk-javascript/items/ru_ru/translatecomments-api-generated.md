Переводит указанные комментарии на язык виджета. Доступно только если соответствующее правило настройки виджета имеет Comment Translation со значением Automatic или Manual.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| commentTranslationsRequest | CommentTranslationsRequest | Да |  |
| sso | string | Нет |  |
| locale | string | Нет |  |

## Ответ

Возвращает: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример translateComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

console.log(`Переведено комментариев: ${response.translations.length}`);
[inline-code-end]