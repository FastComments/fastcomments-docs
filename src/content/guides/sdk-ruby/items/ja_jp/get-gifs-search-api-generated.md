## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| search | string | query | はい |  |
| locale | string | query | いいえ |  |
| rating | string | query | いいえ |  |
| page | number | query | いいえ |  |

## レスポンス

戻り値: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_search_response.rb)

## 例

[inline-code-attrs-start title = 'get_gifs_search の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
search = 'search_example' # 文字列 | 
opts = {
  locale: 'locale_example', # 文字列 | 
  rating: 'rating_example', # 文字列 | 
  page: 1.2 # 浮動小数点数 | 
}

begin
  
  result = api_instance.get_gifs_search(tenant_id, search, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_search: #{e}"
end
[inline-code-end]