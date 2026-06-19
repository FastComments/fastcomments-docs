---
## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| title | string | いいえ |  |

## レスポンス

戻り値: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## 例

[inline-code-attrs-start title = 'createV1PageReact の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-42';
const urlId: string = 'blog/how-we-reduce-latency';
const title: string | undefined = 'Reducing Frontend Latency with FastComments';
const createResponse: CreateV1PageReact = await createV1PageReact(tenantId, urlId, title);
const createResponseNoTitle: CreateV1PageReact = await createV1PageReact(tenantId, urlId);
[inline-code-end]

---