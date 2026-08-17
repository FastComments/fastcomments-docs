## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## レスポンス

返却: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_api_get_comments_response.rb)

## 例

[inline-code-attrs-start title = 'get_api_comments 例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
opts = {
  page: 1.2, # 浮動小数点数 | 
  count: 1.2, # 浮動小数点数 | 
  text_search: 'text_search_example', # 文字列 | 
  by_ip_from_comment: 'by_ip_from_comment_example', # 文字列 | 
  filters: 'filters_example', # 文字列 | 
  search_filters: 'search_filters_example', # 文字列 | 
  sorts: 'sorts_example', # 文字列 | 
  demo: true, # 真偽値 | 
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.get_api_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_comments: #{e}"
end
[inline-code-end]

---