## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| title | string | No |  |

## 응답

반환: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## 예제

[inline-code-attrs-start title = 'createV1PageReact 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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