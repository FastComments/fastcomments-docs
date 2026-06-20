## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| locale | string | query | 否 |  |
| rating | string | query | 否 |  |
| page | number | query | 否 |  |

## 回應

回傳: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_trending_response.rb)

## 範例

[inline-code-attrs-start title = 'get_gifs_trending 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
opts = {
  locale: 'locale_example', # 字串 | 
  rating: 'rating_example', # 字串 | 
  page: 1.2 # 浮點數 | 
}

begin
  
  result = api_instance.get_gifs_trending(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_trending: #{e}"
end
[inline-code-end]

---