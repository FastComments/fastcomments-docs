## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| namespace | string | 是 |  |
| component | string | 是 |  |
| locale | string | 否 |  |
| useFullTranslationIds | boolean | 否 |  |

## 回應

回傳：[`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getTranslations 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---