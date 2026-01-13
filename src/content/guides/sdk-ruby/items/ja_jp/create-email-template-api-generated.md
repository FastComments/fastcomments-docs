## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_email_template200_response.rb)

## 例

[inline-code-attrs-start title = 'create_email_template の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # APIキー認証を構成: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーの接頭辞（例: 'Bearer'）を設定するには、以下の行のコメントアウトを解除してください（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_email_template_body = FastCommentsClient::CreateEmailTemplateBody.new({email_template_id: 'email_template_id_example', display_name: 'display_name_example', ejs: 'ejs_example'}) # CreateEmailTemplateBody | 

begin
  
  result = api_instance.create_email_template(tenant_id, create_email_template_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_email_template: #{e}"
end
[inline-code-end]