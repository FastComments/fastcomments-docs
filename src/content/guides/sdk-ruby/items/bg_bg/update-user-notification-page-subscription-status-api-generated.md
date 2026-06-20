Активиране или деактивиране на известия за страница. Когато потребителите са абонирани за страница, се създават известия за нови основни коментари, както и

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| url | string | query | Да |  |
| pageTitle | string | query | Да |  |
| subscribedOrUnsubscribed | string | path | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_page_subscription_status_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за update_user_notification_page_subscription_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Низ | 
url_id = 'url_id_example' # Низ | 
url = 'url_example' # Низ | 
page_title = 'page_title_example' # Низ | 
subscribed_or_unsubscribed = 'subscribe' # Низ | 
opts = {
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]

---