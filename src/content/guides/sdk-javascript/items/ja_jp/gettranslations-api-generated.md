## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| namespace | string | はい |  |
| component | string | はい |  |
| locale | string | いいえ |  |
| useFullTranslationIds | boolean | いいえ |  |

## レスポンス

戻り値: [`GetTranslations200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslations200Response.ts)

## 例

[inline-code-attrs-start title = 'getTranslations の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const namespaceName: string = 'site-ui';
const componentName: string = 'comment-widget';
const locale: string = 'fr-FR';
const useFullTranslationIds: boolean = true;
const translationsWithLocale: GetTranslations200Response = await getTranslations(namespaceName, componentName, locale, useFullTranslationIds);
const translationsDefault: GetTranslations200Response = await getTranslations(namespaceName, componentName);
[inline-code-end]

---