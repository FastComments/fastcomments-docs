## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| page | number | query | 아니요 |  |
| count | number | query | 아니요 |  |
| text-search | string | query | 아니요 |  |
| byIPFromComment | string | query | 아니요 |  |
| filters | string | query | 아니요 |  |
| searchFilters | string | query | 아니요 |  |
| sorts | string | query | 아니요 |  |
| demo | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comments_response.rb)

## 예제

[inline-code-attrs-start title = 'get_api_comments 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  page: 1.2, # Float | 
  count: 1.2, # Float | 
  text_search: 'text_search_example', # String | 
  by_ip_from_comment: 'by_ip_from_comment_example', # String | 
  filters: 'filters_example', # String | 
  search_filters: 'search_filters_example', # String | 
  sorts: 'sorts_example', # String | 
  demo: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_api_comments(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_comments: #{e}"
end
[inline-code-end]

---