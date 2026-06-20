테넌트의 대량 사용자 정보. userIds를 받아 User / SSOUser의 표시 정보를 반환합니다.
댓글 위젯에서 presence 이벤트로 새로 나타난 사용자를 보강하는 데 사용됩니다.
페이지 컨텍스트 없음: 개인정보 보호가 일괄적으로 적용됩니다(비공개 프로필은 마스킹됩니다).

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| ids | string | query | 예 | 쉼표로 구분된 userIds. |

## 응답

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## 예제

[inline-code-attrs-start title = 'get_users_info 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | 쉼표로 구분된 userIds.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---