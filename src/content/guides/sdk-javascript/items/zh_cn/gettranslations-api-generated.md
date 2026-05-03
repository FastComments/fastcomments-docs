## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| namespace | string | 是 |  |
| component | string | 是 |  |
| locale | string | 否 |  |
| useFullTranslationIds | boolean | 否 |  |

## 响应

返回：[`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getTranslations 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---