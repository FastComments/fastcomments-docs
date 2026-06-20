## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | 否 |  |
| count | number | query | 否 |  |
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sorts | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comments_response.rb)

## 範例

[inline-code-attrs-start title = 'get_api_comments 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  page: 1.2, # 浮點數 | 
  count: 1.2, # 浮點數 | 
  text_search: 'text_search_example', # 字串 | 
  by_ip_from_comment: 'by_ip_from_comment_example', # 字串 | 
  filters: 'filters_example', # 字串 | 
  search_filters: 'search_filters_example', # 字串 | 
  sorts: 'sorts_example', # 字串 | 
  demo: true, # 布林值 | 
  sso: 'sso_example' # 字串 | 
}

begin
  
  result = api_instance.get_api_comments(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_comments: #{e}"
end
[inline-code-end]