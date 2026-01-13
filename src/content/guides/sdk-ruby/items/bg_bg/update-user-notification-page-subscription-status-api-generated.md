Активиране или деактивиране на известия за страница. Когато потребителите са абонирани за страница, се създават известия за нови основни коментари, и също така

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| url | string | query | Да |  |
| pageTitle | string | query | Да |  |
| subscribedOrUnsubscribed | string | path | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## Пример

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
url = 'url_example' # String | 
page_title = 'page_title_example' # String | 
subscribed_or_unsubscribed = 'subscribe' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]