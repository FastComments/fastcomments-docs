---
## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
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

## 响应

返回: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comments_response.rb)

## 示例

[inline-code-attrs-start title = 'get_api_comments 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  page: 1.2, # 浮点数 | 
  count: 1.2, # 浮点数 | 
  text_search: 'text_search_example', # 字符串 | 
  by_ip_from_comment: 'by_ip_from_comment_example', # 字符串 | 
  filters: 'filters_example', # 字符串 | 
  search_filters: 'search_filters_example', # 字符串 | 
  sorts: 'sorts_example', # 字符串 | 
  demo: true, # 布尔值 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.get_api_comments(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_comments: #{e}"
end
[inline-code-end]

---