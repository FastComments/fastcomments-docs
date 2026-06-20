## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| userId | string | クエリ | いいえ |  |
| state | number | クエリ | いいえ |  |
| skip | number | クエリ | いいえ |  |
| limit | number | クエリ | いいえ |  |

## レスポンス

戻り値: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tickets_response.rb)

## 例

[inline-code-attrs-start title = 'get_tickets の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーにプレフィックス（例: 'Bearer'）を設定するには、以下の行のコメントを外してください（デフォルトはnil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  state: 1.2, # Float | 
  skip: 1.2, # Float | 
  limit: 1.2 # Float | 
}

begin
  
  result = api_instance.get_tickets(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tickets: #{e}"
end
[inline-code-end]

---