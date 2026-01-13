## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |

## 响应

返回: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_votes200_response.rb)

## 示例

[inline-code-attrs-start title = 'get_votes 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
url_id = 'url_id_example' # String | 

begin
  
  result = api_instance.get_votes(tenant_id, url_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_votes: #{e}"
end
[inline-code-end]