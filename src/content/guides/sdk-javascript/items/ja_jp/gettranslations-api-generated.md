## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| namespace | string | はい |  |
| component | string | はい |  |
| locale | string | いいえ |  |
| useFullTranslationIds | boolean | いいえ |  |

## レスポンス

返却: [`GetTranslationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse1.ts)

## 例

[inline-code-attrs-start title = 'getTranslations の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const namespace: string = 'blog';
  const component: string = 'comment-editor';
  const locale: string = 'fr-FR';
  const useFullTranslationIds: boolean = true;

  const basicTranslations: GetTranslationsResponse1 = await getTranslations(namespace, component);
  const fullTranslations: GetTranslationsResponse1 = await getTranslations(namespace, component, locale, useFullTranslationIds);
})();
[inline-code-end]

---