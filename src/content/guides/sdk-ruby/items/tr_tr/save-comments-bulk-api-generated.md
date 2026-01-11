## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| isLive | boolean | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| sendEmails | boolean | query | Hayır |  |
| populateNotifications | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`Array&lt;SaveComment200Response&gt;`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/array&lt;_save_comment200_response&gt;.rb)

## Örnek

[inline-code-attrs-start title = 'save_comments_bulk Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirmeyi ayarla
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API anahtarı için bir önek ayarlamak üzere aşağıdaki satırın başındaki # işaretini kaldırın, ör. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_comment_params = [FastCommentsClient::CreateCommentParams.new({commenter_name: 'commenter_name_example', comment: 'comment_example', url: 'url_example', url_id: 'url_id_example', locale: 'locale_example'})] # Array<CreateCommentParams> | 
opts = {
  is_live: true, # Boolean | 
  do_spam_check: true, # Boolean | 
  send_emails: true, # Boolean | 
  populate_notifications: true # Boolean | 
}

begin
  
  result = api_instance.save_comments_bulk(tenant_id, create_comment_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->save_comments_bulk: #{e}"
end
[inline-code-end]