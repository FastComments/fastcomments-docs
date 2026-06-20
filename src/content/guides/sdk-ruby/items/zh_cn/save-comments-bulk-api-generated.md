## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| isLive | boolean | query | 否 |  |
| doSpamCheck | boolean | query | 否 |  |
| sendEmails | boolean | query | 否 |  |
| populateNotifications | boolean | query | 否 |  |

## 响应

返回: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/save_comments_bulk_response.rb)

## 示例

[inline-code-attrs-start title = 'save_comments_bulk 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 设置授权
FastCommentsClient.configure do |config|
  # 配置 API 密钥授权：api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消注释以下行以为 API 密钥设置前缀，例如 'Bearer'（默认为 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
create_comment_params = [FastCommentsClient::CreateCommentParams.new({commenter_name: 'commenter_name_example', comment: 'comment_example', url: 'url_example', url_id: 'url_id_example', locale: 'locale_example'})] # Array<CreateCommentParams> | 
opts = {
  is_live: true, # 布尔值 | 
  do_spam_check: true, # 布尔值 | 
  send_emails: true, # 布尔值 | 
  populate_notifications: true # 布尔值 | 
}

begin
  
  result = api_instance.save_comments_bulk(tenant_id, create_comment_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->save_comments_bulk: #{e}"
end
[inline-code-end]