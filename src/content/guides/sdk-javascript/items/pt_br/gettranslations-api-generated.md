## Parâmetros

| Name | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| namespace | string | Sim |  |
| component | string | Sim |  |
| locale | string | Não |  |
| useFullTranslationIds | boolean | Não |  |

## Resposta

Retorna: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---