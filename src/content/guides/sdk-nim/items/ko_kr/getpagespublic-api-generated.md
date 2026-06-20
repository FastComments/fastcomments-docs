테넌트의 페이지 목록을 반환합니다. FChat 데스크탑 클라이언트에서 룸 목록을 채우기 위해 사용됩니다.
`enableFChat`가 각 페이지의 해결된 커스텀 구성에서 true여야 합니다.
SSO가 필요한 페이지는 요청하는 사용자의 그룹 접근 권한을 기준으로 필터링됩니다.

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | int | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | bool | No |  |

## 응답

반환: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(
  tenantId = "my-tenant-123",
  cursor = "",
  limit = 0,
  q = "",
  sortBy = PagesSortBy(0),
  hasComments = false
)

if response.isSome:
  let pages = response.get()
  echo "Retrieved public pages: ", $pages
else:
  echo "No pages returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---