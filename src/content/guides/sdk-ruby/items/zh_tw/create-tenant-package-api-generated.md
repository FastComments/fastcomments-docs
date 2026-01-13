## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_tenant_package200_response.rb)

## 範例

[inline-code-attrs-start title = 'create_tenant_package 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 設定授權
FastCommentsClient.configure do |config|
  # 設定 API 金鑰授權: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消註解下列行以設定 API 金鑰的前綴，例如 'Bearer' (defaults to nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_tenant_package_body = FastCommentsClient::CreateTenantPackageBody.new({name: 'name_example', max_monthly_page_loads: 3.56, max_monthly_api_credits: 3.56, max_monthly_comments: 3.56, max_concurrent_users: 3.56, max_tenant_users: 3.56, max_sso_users: 3.56, max_moderators: 3.56, max_domains: 3.56, has_debranding: false, for_who_text: 'for_who_text_example', feature_taglines: ['feature_taglines_example'], has_flex_pricing: false}) # CreateTenantPackageBody | 

begin
  
  result = api_instance.create_tenant_package(tenant_id, create_tenant_package_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_tenant_package: #{e}"
end
[inline-code-end]