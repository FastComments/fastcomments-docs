## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## 示例

[inline-code-attrs-start title = 'update_email_template 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 设置授权
FastCommentsClient.configure do |config|
  # 配置 API 密钥授权：api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消注释以下行以为 API 密钥设置前缀，例如 'Bearer'（默认值为 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_email_template_body = FastCommentsClient::UpdateEmailTemplateBody.new # UpdateEmailTemplateBody | 

begin
  
  result = api_instance.update_email_template(tenant_id, id, update_email_template_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_email_template: #{e}"
end
[inline-code-end]