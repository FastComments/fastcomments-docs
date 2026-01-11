## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_page_a_p_i_response.rb)

## 例

[inline-code-attrs-start title = 'patch_page の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーにプレフィックスを設定するには次の行のコメントを外してください（例: 'Bearer'、デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_api_page_data = FastCommentsClient::UpdateAPIPageData.new # UpdateAPIPageData | 

begin
  
  result = api_instance.patch_page(tenant_id, id, update_api_page_data)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_page: #{e}"
end
[inline-code-end]

---