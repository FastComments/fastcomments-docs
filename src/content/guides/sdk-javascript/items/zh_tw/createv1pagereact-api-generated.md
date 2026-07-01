## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| title | string | 否 |  |

## 回應

返回: [`CreateV1PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReactResponse.ts)

## 範例

[inline-code-attrs-start title = 'createV1PageReact 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-2024-06-improvements";
const title: string = "FastComments API Integration Guide";

const responseWithTitle: CreateV1PageReactResponse = await createV1PageReact(tenantId, urlId, title);
const responseWithoutTitle: CreateV1PageReactResponse = await createV1PageReact(tenantId, urlId);
[inline-code-end]