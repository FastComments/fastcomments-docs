## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| namespace | string | 是 |  |
| component | string | 是 |  |
| locale | string | 否 |  |
| useFullTranslationIds | boolean | 否 |  |

## 响应

返回: [`GetTranslations200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslations200Response.ts)

## 示例

[inline-code-attrs-start title = 'getTranslations 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const namespaceName: string = 'site-ui';
const componentName: string = 'comment-widget';
const locale: string = 'fr-FR';
const useFullTranslationIds: boolean = true;
const translationsWithLocale: GetTranslations200Response = await getTranslations(namespaceName, componentName, locale, useFullTranslationIds);
const translationsDefault: GetTranslations200Response = await getTranslations(namespaceName, componentName);
[inline-code-end]

---