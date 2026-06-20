---
## 参数

| 名称 | Type | Location | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| id | string | query | 是 |  |
| title | string | query | 否 |  |

## 响应

返回: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_v1_page_react.rb)

## 示例

[inline-code-attrs-start title = 'create_v2_page_react 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
url_id = 'url_id_example' # 字符串 | 
id = 'id_example' # 字符串 | 
opts = {
  title: 'title_example' # 字符串 | 
}

begin
  
  result = api_instance.create_v2_page_react(tenant_id, url_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->create_v2_page_react: #{e}"
end
[inline-code-end]

---