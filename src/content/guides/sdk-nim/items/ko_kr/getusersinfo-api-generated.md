---
테넌트의 대량 사용자 정보. userIds가 주어지면 User / SSOUser로부터 표시 정보를 반환합니다.
댓글 위젯에서 presence event로 방금 등장한 사용자를 보강하는 데 사용됩니다.
페이지 컨텍스트 없음: 개인 정보 보호가 일률적으로 적용됩니다(비공개 프로필은 마스킹됨).

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| ids | string | 아니요 |  |

## 응답

반환: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## 예제

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---