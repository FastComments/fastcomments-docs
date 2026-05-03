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
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---