## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## 示例

[inline-code-attrs-start title = 'update_tenant_package 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 设置授权
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消注释以下行以为 API 密钥设置前缀，例如 'Bearer'（默认为 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_tenant_package_body = FastCommentsClient::UpdateTenantPackageBody.new # UpdateTenantPackageBody | 

begin
  
  result = api_instance.update_tenant_package(tenant_id, id, update_tenant_package_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_tenant_package: #{e}"
end
[inline-code-end]