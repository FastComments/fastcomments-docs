---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_comment_search_response.rb)

## 示例

[inline-code-attrs-start title = 'get_search_comments_summary 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # String | 
  filters: 'filters_example', # String | 
  search_filters: 'search_filters_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_search_comments_summary(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_comments_summary: #{e}"
end
[inline-code-end]

---