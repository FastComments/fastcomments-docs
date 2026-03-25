## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| state | number | query | 否 |  |
| skip | number | query | 否 |  |
| limit | number | query | 否 |  |

## 回應

回傳: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tickets200_response.rb)

## 範例

[inline-code-attrs-start title = 'get_tickets 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 設定授權
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消註解下列程式碼行以設定 API 金鑰的前綴，例如 'Bearer'（預設為 nil）
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