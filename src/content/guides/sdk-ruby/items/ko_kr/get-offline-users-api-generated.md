---
현재 온라인 상태가 아닌 해당 페이지의 이전 댓글 작성자들입니다. displayName으로 정렬됩니다.
"/users/online"을 모두 사용한 후 "구성원" 섹션을 렌더링하기 위해 이것을 사용하세요.
commenterName에 대한 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName} 인덱스를 afterName부터 $gt로 앞으로 탐색하며 $skip 비용이 없습니다.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | No | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | No | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 같은 항목이 누락되지 않도록 필요합니다. |

## Response

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Example

[inline-code-attrs-start title = 'get_offline_users 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 페이지 URL 식별자(서버 측에서 정리됨).
opts = {
  after_name: 'after_name_example', # String | 커서: 이전 응답의 nextAfterName을 전달하세요.
  after_user_id: 'after_user_id_example' # String | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 동일한 항목이 누락되지 않도록 필요합니다.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---