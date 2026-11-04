테넌트의 대량 사용자 정보입니다. userIds가 주어지면 User / SSOUser에서 표시 정보를 반환합니다.
presence 이벤트를 통해 방금 나타난 사용자를 풍부하게 하기 위해 댓글 위젯에서 사용됩니다.
페이지 컨텍스트가 없습니다: 프라이버시는 일관되게 적용됩니다(비공개 프로필은 마스킹됩니다).

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| ids | string | 아니요 |  |

## 응답

반환값: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## 예제

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]