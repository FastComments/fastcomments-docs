## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 예 |  |
| direction | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| anonUserId | string | query | 아니오 |  |

## 응답

반환: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_comment200_response.rb)

## 예제

[inline-code-attrs-start title = 'create_vote 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 접두어를 설정하려면 다음 줄의 주석을 해제하세요. 예: 'Bearer' (기본값: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
comment_id = 'comment_id_example' # 문자열 | 
direction = 'up' # 문자열 | 
opts = {
  user_id: 'user_id_example', # 문자열 | 
  anon_user_id: 'anon_user_id_example' # 문자열 | 
}

begin
  
  result = api_instance.create_vote(tenant_id, comment_id, direction, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_vote: #{e}"
end
[inline-code-end]

---