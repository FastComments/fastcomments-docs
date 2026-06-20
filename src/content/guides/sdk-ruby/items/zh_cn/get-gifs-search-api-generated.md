## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| search | string | query | 是 |  |
| locale | string | query | 否 |  |
| rating | string | query | 否 |  |
| page | number | query | 否 |  |

## 响应

返回: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_search_response.rb)

## 示例

[inline-code-attrs-start title = 'get_gifs_search 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
search = 'search_example' # 字符串 | 
opts = {
  locale: 'locale_example', # 字符串 | 
  rating: 'rating_example', # 字符串 | 
  page: 1.2 # 浮点数 | 
}

begin
  
  result = api_instance.get_gifs_search(tenant_id, search, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_search: #{e}"
end
[inline-code-end]