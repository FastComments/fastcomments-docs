## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| namespace | string | 是 |  |
| component | string | 是 |  |
| locale | string | 否 |  |
| useFullTranslationIds | boolean | 否 |  |

## 回應

Returns: [`GetTranslationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getTranslations 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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