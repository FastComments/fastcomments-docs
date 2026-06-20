현재 페이지에 온라인 상태인 뷰어: 웹소켓 세션이 현재 해당 페이지에 구독된 사람들.
anonCount + totalCount를 반환합니다 (room-wide 구독자, 익명 뷰어는 개별 열거하지 않음).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | 아니요 | 커서: 이전 응답에서 nextAfterName을 전달하세요. |
| afterUserId | string | query | 아니요 | 커서 티브레이커: 이전 응답에서 nextAfterUserId를 전달하세요. afterName이 설정된 경우, 이름 동명이인으로 항목이 누락되지 않도록 필요합니다. |

## Response

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Example

[inline-code-attrs-start title = 'get_online_users 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 페이지 URL 식별자 (서버 측에서 정리됨).
opts = {
  after_name: 'after_name_example', # String | 커서: 이전 응답에서 nextAfterName을 전달하세요.
  after_user_id: 'after_user_id_example' # String | 커서 티브레이커: 이전 응답에서 nextAfterUserId를 전달하세요. afterName이 설정된 경우, 이름 동명이인으로 항목이 누락되지 않도록 필요합니다.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---