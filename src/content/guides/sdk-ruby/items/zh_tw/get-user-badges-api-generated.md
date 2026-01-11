## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| badgeId | string | query | 否 |  |
| type | number | query | 否 |  |
| displayedOnComments | boolean | query | 否 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳： [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_badges200_response.rb)

## 範例

[inline-code-attrs-start title = 'get_user_badges 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 設定授權
FastCommentsClient.configure do |config|
  # 設定 API 金鑰授權：api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消註解下列程式行以為 API 金鑰設定前綴，例如 'Bearer'（預設為 nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 字串 | 
opts = {
  user_id: 'user_id_example', # 字串 | 
  badge_id: 'badge_id_example', # 字串 | 
  type: 1.2, # 浮點數 | 
  displayed_on_comments: true, # 布林值 | 
  limit: 1.2, # 浮點數 | 
  skip: 1.2 # 浮點數 | 
}

begin
  
  result = api_instance.get_user_badges(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badges: #{e}"
end
[inline-code-end]

---