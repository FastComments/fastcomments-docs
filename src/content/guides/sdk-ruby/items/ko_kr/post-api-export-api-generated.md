## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| text-search | string | query | 아니요 |  |
| byIPFromComment | string | query | 아니요 |  |
| filters | string | query | 아니요 |  |
| searchFilters | string | query | 아니요 |  |
| sorts | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_export_response.rb)

## 예제

[inline-code-attrs-start title = 'post_api_export 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # String | 
  by_ip_from_comment: 'by_ip_from_comment_example', # String | 
  filters: 'filters_example', # String | 
  search_filters: 'search_filters_example', # String | 
  sorts: 'sorts_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_api_export(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_api_export: #{e}"
end
[inline-code-end]