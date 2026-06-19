## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| namespace | string | Да |  |
| component | string | Да |  |
| locale | string | Не |  |
| useFullTranslationIds | boolean | Не |  |

## Одговор

Враћа: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---