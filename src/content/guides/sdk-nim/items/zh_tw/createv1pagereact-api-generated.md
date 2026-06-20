## 參數

| 名稱 | 型別 | 是否必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| title | string | 否 |  |

## 回傳

回傳：[`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## 範例

[inline-code-attrs-start title = 'createV1PageReact 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createV1PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/2026/market-rally",
  title = "Breaking News: Markets Rally Today"
)

if response.isSome:
  let pageReact = response.get()
  echo "Page react created: ", pageReact
else:
  echo "Failed to create page react: ", httpResponse
[inline-code-end]

---