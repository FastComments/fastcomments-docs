## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| namespace | string | Да |  |
| component | string | Да |  |
| locale | string | Не |  |
| useFullTranslationIds | boolean | Не |  |

## Одговор

Враћа: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Пример

[inline-code-attrs-start title = 'getTranslations Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---