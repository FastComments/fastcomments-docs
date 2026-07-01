테넌트의 페이지 목록을 조회합니다. FChat 데스크톱 클라이언트가 방 목록을 채우는 데 사용됩니다.  
`enableFChat`이 각 페이지에 대한 해결된 사용자 지정 구성에서 true로 설정되어 있어야 합니다.  
SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetPagesPublicOptions | No |  |

## 응답

반환: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## 예시

[inline-code-attrs-start title = 'getPagesPublic 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]

---