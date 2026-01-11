## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| skip | integer | query | いいえ |  |

## レスポンス

戻り値: [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_s_s_o_users200_response.rb)

## 例

[inline-code-attrs-start title = 'get_sso_users の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証のセットアップ
FastCommentsClient.configure do |config|
  # API キー認証の設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API キーにプレフィックスを設定するには、次の行のコメントを外してください。例: 'Bearer' (デフォルトは nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 56 # Integer | 
}

begin
  
  result = api_instance.get_sso_users(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_sso_users: #{e}"
end
[inline-code-end]