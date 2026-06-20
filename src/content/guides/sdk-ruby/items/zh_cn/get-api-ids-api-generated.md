## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| afterId | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comment_ids_response.rb)

## 示例

[inline-code-attrs-start title = 'get_api_ids 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # 字符串 | 
  by_ip_from_comment: 'by_ip_from_comment_example', # 字符串 | 
  filters: 'filters_example', # 字符串 | 
  search_filters: 'search_filters_example', # 字符串 | 
  after_id: 'after_id_example', # 字符串 | 
  demo: true, # 布尔值 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.get_api_ids(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_ids: #{e}"
end
[inline-code-end]

---