## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| skip | number | query | 否 |  |

## 回應

回傳：[`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template_render_errors_response.rb)

## 範例

[inline-code-attrs-start title = 'get_email_template_render_errors 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 設定授權
FastCommentsClient.configure do |config|
  # 設定 API 金鑰授權：api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消註解下列行以設定 API 金鑰的前綴，例如 'Bearer' (預設為 nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 字串 | 
id = 'id_example' # 字串 | 
opts = {
  skip: 1.2 # 浮點數 | 
}

begin
  
  result = api_instance.get_email_template_render_errors(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template_render_errors: #{e}"
end
[inline-code-end]

---