## Параметри

| Назва | Тип | Обов'язково | Опис |
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
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---