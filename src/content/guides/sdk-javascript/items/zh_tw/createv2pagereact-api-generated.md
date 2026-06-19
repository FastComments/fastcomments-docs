## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| id | string | 是 |  |
| title | string | 否 |  |

## 回應

回傳：[`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## 範例

[inline-code-attrs-start title = 'createV2PageReact 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_91f4b3b';
const urlId: string = 'https://news.site.com/articles/2026/06/fastcomments-integration';
const id: string = 'react_5f2c1a';
const title: string = 'FastComments Integration — June 2026';

const reactionWithTitle: CreateV1PageReact = await createV2PageReact(tenantId, urlId, id, title);
const reactionWithoutTitle: CreateV1PageReact = await createV2PageReact(tenantId, urlId, id);
[inline-code-end]

---