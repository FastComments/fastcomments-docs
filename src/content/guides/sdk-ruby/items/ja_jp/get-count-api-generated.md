## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| text-search | string | query | いいえ |  |
| byIPFromComment | string | query | いいえ |  |
| filter | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| demo | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_count_comments_response.rb)

## 例

[inline-code-attrs-start title = 'get_count の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # 文字列 | 
  by_ip_from_comment: 'by_ip_from_comment_example', # 文字列 | 
  filter: 'filter_example', # 文字列 | 
  search_filters: 'search_filters_example', # 文字列 | 
  demo: true, # 真偽値 | 
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.get_count(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_count: #{e}"
end
[inline-code-end]

---