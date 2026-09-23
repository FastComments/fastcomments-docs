## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| title | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## 示例

[inline-code-attrs-start title = 'createV1PageReact 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "my-tenant-123";
const urlId: string = "article-456";

const pageTitle: string = "Understanding TypeScript Generics";
const ssoToken: string = "user-789-token";

const fullResult: CreateV1PageReact = await createV1PageReact(tenantId, urlId, pageTitle, ssoToken);
const minimalResult: CreateV1PageReact = await createV1PageReact(tenantId, urlId);
[inline-code-end]