Activer ou désactiver les notifications pour une page. Lorsque les utilisateurs sont abonnés à une page, des notifications sont créées pour les nouveaux commentaires racines, et aussi

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Oui |  |
| url | string | query | Oui |  |
| pageTitle | string | query | Oui |  |
| subscribedOrUnsubscribed | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_user_notification_page_subscription_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
url_id = 'url_id_example' # Chaîne | 
url = 'url_example' # Chaîne | 
page_title = 'page_title_example' # Chaîne | 
subscribed_or_unsubscribed = 'subscribe' # Chaîne | 
opts = {
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]