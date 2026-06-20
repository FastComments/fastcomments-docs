req
tenantId
afterId

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | 文字列 | パス | はい |  |
| afterId | 文字列 | クエリ | いいえ |  |
| limit | 整数 | クエリ | いいえ |  |
| tags | 配列 | クエリ | いいえ |  |
| sso | 文字列 | クエリ | いいえ |  |
| isCrawler | 真偽値 | クエリ | いいえ |  |
| includeUserInfo | 真偽値 | クエリ | いいえ |  |

## レスポンス

戻り値: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_feed_posts_response.rb)

## 例

[inline-code-attrs-start title = 'get_feed_posts_public の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
opts = {
  after_id: 'after_id_example', # 文字列 | 
  limit: 56, # 整数 | 
  tags: ['inner_example'], # 配列<文字列> | 
  sso: 'sso_example', # 文字列 | 
  is_crawler: true, # 真偽値 | 
  include_user_info: true # 真偽値 | 
}

begin
  
  result = api_instance.get_feed_posts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_feed_posts_public: #{e}"
end
[inline-code-end]

---