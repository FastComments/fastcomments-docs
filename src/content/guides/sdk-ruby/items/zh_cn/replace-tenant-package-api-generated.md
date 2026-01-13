## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## 示例

[inline-code-attrs-start title = 'replace_tenant_package 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 设置授权
FastCommentsClient.configure do |config|
  # 配置 API 密钥授权: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消注释下面的行以为 API 密钥设置前缀，例如 'Bearer'（默认值为 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
replace_tenant_package_body = FastCommentsClient::ReplaceTenantPackageBody.new({name: 'name_example', monthly_cost_usd: 3.56, yearly_cost_usd: 3.56, max_monthly_page_loads: 3.56, max_monthly_api_credits: 3.56, max_monthly_comments: 3.56, max_concurrent_users: 3.56, max_tenant_users: 3.56, max_sso_users: 3.56, max_moderators: 3.56, max_domains: 3.56, has_debranding: false, for_who_text: 'for_who_text_example', feature_taglines: ['feature_taglines_example'], has_flex_pricing: false}) # ReplaceTenantPackageBody | 

begin
  
  result = api_instance.replace_tenant_package(tenant_id, id, replace_tenant_package_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->replace_tenant_package: #{e}"
end
[inline-code-end]