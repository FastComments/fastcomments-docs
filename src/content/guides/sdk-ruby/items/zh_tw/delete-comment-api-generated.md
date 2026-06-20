## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | Yes |  |
| id | string | 路徑 | Yes |  |
| contextUserId | string | 查詢 | No |  |
| isLive | boolean | 查詢 | No |  |

## 回應

回傳: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_result.rb)

## 範例

[inline-code-attrs-start title = 'delete_comment 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 設定授權
FastCommentsClient.configure do |config|
  # 設定 API key 授權: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消註解以下一行以為 API key 設定前綴，例如 'Bearer'（預設為 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  context_user_id: 'context_user_id_example', # String | 
  is_live: true # Boolean | 
}

begin
  
  result = api_instance.delete_comment(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_comment: #{e}"
end
[inline-code-end]