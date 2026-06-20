## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| text-search | string | query | 아니오 |  |
| byIPFromComment | string | query | 아니오 |  |
| filter | string | query | 아니오 |  |
| searchFilters | string | query | 아니오 |  |
| demo | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_count_comments_response.rb)

## 예제

[inline-code-attrs-start title = 'get_count 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # 문자열 | 
  by_ip_from_comment: 'by_ip_from_comment_example', # 문자열 | 
  filter: 'filter_example', # 문자열 | 
  search_filters: 'search_filters_example', # 문자열 | 
  demo: true, # 불리언 | 
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.get_count(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_count: #{e}"
end
[inline-code-end]