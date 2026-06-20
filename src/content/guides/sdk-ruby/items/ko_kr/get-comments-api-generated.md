## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## 응답

반환: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_comments_response.rb)

## 예제

[inline-code-attrs-start title = 'get_comments 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 접두사를 설정하려면 다음 줄의 주석을 해제하세요. 예: 'Bearer' (기본값: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
opts = {
  page: 56, # 정수 | 
  limit: 56, # 정수 | 
  skip: 56, # 정수 | 
  as_tree: true, # 부울 | 
  skip_children: 56, # 정수 | 
  limit_children: 56, # 정수 | 
  max_tree_depth: 56, # 정수 | 
  url_id: 'url_id_example', # 문자열 | 
  user_id: 'user_id_example', # 문자열 | 
  anon_user_id: 'anon_user_id_example', # 문자열 | 
  context_user_id: 'context_user_id_example', # 문자열 | 
  hash_tag: 'hash_tag_example', # 문자열 | 
  parent_id: 'parent_id_example', # 문자열 | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  from_date: 789, # 정수 | 
  to_date: 789 # 정수 | 
}

begin
  
  result = api_instance.get_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_comments: #{e}"
end
[inline-code-end]