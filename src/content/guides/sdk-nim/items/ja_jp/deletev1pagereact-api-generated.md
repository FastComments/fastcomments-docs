## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## レスポンス

返却: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## 例

[inline-code-attrs-start title = 'deleteV1PageReact の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (reactOpt, httpResp) = client.deleteV1PageReact(tenantId = "my-tenant-123", urlId = "news/article-title")
if reactOpt.isSome:
  let react = reactOpt.get()
[inline-code-end]