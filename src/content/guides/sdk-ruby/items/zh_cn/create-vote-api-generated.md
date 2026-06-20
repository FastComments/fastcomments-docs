## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 是 |  |
| direction | string | query | 是 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |

## 返回

返回： [`VoteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_response.rb)

## 示例

[inline-code-attrs-start title = 'create_vote 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 设置授权
FastCommentsClient.configure do |config|
  # 配置 API 密钥授权: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消注释以下行以为 API 密钥设置前缀，例如 'Bearer'（默认为 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
comment_id = 'comment_id_example' # 字符串 | 
direction = 'up' # 字符串 | 
opts = {
  user_id: 'user_id_example', # 字符串 | 
  anon_user_id: 'anon_user_id_example' # 字符串 | 
}

begin
  
  result = api_instance.create_vote(tenant_id, comment_id, direction, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_vote: #{e}"
end
[inline-code-end]

---