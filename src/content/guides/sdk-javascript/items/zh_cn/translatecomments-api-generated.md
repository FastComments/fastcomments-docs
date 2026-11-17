将给定的评论翻译为小组件的语言环境。仅当匹配的小组件自定义规则中的“评论翻译”设置为“自动”或“手动”时才可用。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| commentTranslationsRequest | CommentTranslationsRequest | 是 |  |
| sso | string | 否 |  |
| locale | string | 否 |  |

## 响应

返回:[`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## 示例

[inline-code-attrs-start title = 'translateComments 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'blog-post-42';
const sso: string = 'JWT_TOKEN_HERE';

const commentTranslationsRequest: CommentTranslationsRequest = {
  commentIds: ['c_98765', 'c_54321'],
  targetLocale: 'es'
};

const locale: string = 'es-ES';

const response: CommentTranslationsResponse = await translateComments(
  tenantId,
  urlId,
  commentTranslationsRequest,
  sso,
  locale
);

console.log(`Translated ${response.translations.length} comments`);
[inline-code-end]