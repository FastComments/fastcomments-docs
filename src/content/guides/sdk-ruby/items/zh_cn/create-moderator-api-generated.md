## 参数

| 名称 | 类型 | 位置 | 必需 | 说明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 响应

返回：[`CreateModerator200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_moderator200_response.rb)

## 示例

[inline-code-attrs-start title = 'create_moderator 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 设置授权
FastCommentsClient.configure do |config|
  # 配置 API 密钥授权: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消注释以下行以为 API 密钥设置前缀，例如 'Bearer'（默认值为 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_moderator_body = FastCommentsClient::CreateModeratorBody.new({name: 'name_example', email: 'email_example'}) # CreateModeratorBody | 

begin
  
  result = api_instance.create_moderator(tenant_id, create_moderator_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_moderator: #{e}"
end
[inline-code-end]