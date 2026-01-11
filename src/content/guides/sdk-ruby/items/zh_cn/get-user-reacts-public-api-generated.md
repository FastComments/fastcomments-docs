## 参数

| Name | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postIds | array | query | No |  |
| sso | string | query | No |  |

## 响应

返回：[`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_reacts_public200_response.rb)

## 示例

[inline-code-attrs-start title = 'get_user_reacts_public 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
opts = {
  post_ids: ['inner_example'], # 数组<String> | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.get_user_reacts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_reacts_public: #{e}"
end
[inline-code-end]

---