## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| namespace | string | Да |  |
| component | string | Да |  |
| locale | string | Нет |  |
| useFullTranslationIds | boolean | Нет |  |

## Ответ

Возвращает: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsDefault: GetTranslationsResponse = await getTranslations("payments", "checkout");
const translationsFrenchDetailed: GetTranslationsResponse = await getTranslations("payments", "checkout", "fr-FR", true);
[inline-code-end]

---