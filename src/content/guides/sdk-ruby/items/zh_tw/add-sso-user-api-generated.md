## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 回應

回傳: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_s_s_o_user_a_p_i_response.rb)

## 範例

[inline-code-attrs-start title = 'add_sso_user 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
tenant_id = 'tenant_id_example' # 字串 | 
create_apisso_user_data = FastCommentsClient::CreateAPISSOUserData.new({email: 'email_example', username: 'username_example', id: 'id_example'}) # CreateAPISSOUserData | 

begin
  
  result = api_instance.add_sso_user(tenant_id, create_apisso_user_data)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_sso_user: #{e}"
end
[inline-code-end]

---