---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | query | いいえ |  |
| externalId | string | query | いいえ |  |
| eventType | string | query | いいえ |  |
| type | string | query | いいえ |  |
| domain | string | query | いいえ |  |
| attemptCountGT | number | query | いいえ |  |

## レスポンス

戻り値: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_pending_webhook_event_count_response.rb)

## 例

[inline-code-attrs-start title = 'get_pending_webhook_event_count の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # API キー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーにプレフィックスを設定するには以下の行のコメントアウトを外します。例: 'Bearer'（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  comment_id: 'comment_id_example', # String | 
  external_id: 'external_id_example', # String | 
  event_type: 'event_type_example', # String | 
  type: 'type_example', # String | 
  domain: 'domain_example', # String | 
  attempt_count_gt: 1.2 # Float | 
}

begin
  
  result = api_instance.get_pending_webhook_event_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_pending_webhook_event_count: #{e}"
end
[inline-code-end]

---