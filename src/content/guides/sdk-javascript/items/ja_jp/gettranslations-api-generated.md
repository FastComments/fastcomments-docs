## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| namespace | string | はい |  |
| component | string | はい |  |
| locale | string | いいえ |  |
| useFullTranslationIds | boolean | いいえ |  |

## レスポンス

返却値: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## 例

[inline-code-attrs-start title = 'getTranslations の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---