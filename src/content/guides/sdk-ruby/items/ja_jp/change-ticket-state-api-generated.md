## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

返却値: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/change_ticket_state200_response.rb)

## 例

[inline-code-attrs-start title = 'change_ticket_state の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 次の行のコメントを外すとAPIキーのプレフィックスを設定できます。例: 'Bearer'（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 
id = 'id_example' # String | 
change_ticket_state_body = FastCommentsClient::ChangeTicketStateBody.new({state: 37}) # ChangeTicketStateBody | 

begin
  
  result = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->change_ticket_state: #{e}"
end
[inline-code-end]

---