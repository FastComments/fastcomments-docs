## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template200_response.rb)

## 範例

[inline-code-attrs-start title = 'get_email_template 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 設定授權
FastCommentsClient.configure do |config|
  # 設定 API 金鑰授權: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消註解下列行以為 API 金鑰設定前綴，例如 'Bearer'（預設為 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_email_template(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template: #{e}"
end
[inline-code-end]

---