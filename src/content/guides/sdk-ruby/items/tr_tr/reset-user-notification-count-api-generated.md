## Parametreler

| İsim | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/reset_user_notifications_response.rb)

## Örnek

[inline-code-attrs-start title = 'reset_user_notification_count Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Dize | 
opts = {
  sso: 'sso_example' # Dize | 
}

begin
  
  result = api_instance.reset_user_notification_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->reset_user_notification_count: #{e}"
end
[inline-code-end]