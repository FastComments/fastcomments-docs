## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| locale | string | query | いいえ |  |

## レスポンス

戻り値: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/render_email_template200_response.rb)

## 例

[inline-code-attrs-start title = 'render_email_template の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証のセットアップ
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 以下の行のコメントを外して APIキーにプレフィックスを設定してください。例: 'Bearer'（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
render_email_template_body = FastCommentsClient::RenderEmailTemplateBody.new({email_template_id: 'email_template_id_example', ejs: 'ejs_example'}) # RenderEmailTemplateBody | 
opts = {
  locale: 'locale_example' # String | 
}

begin
  
  result = api_instance.render_email_template(tenant_id, render_email_template_body, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->render_email_template: #{e}"
end
[inline-code-end]