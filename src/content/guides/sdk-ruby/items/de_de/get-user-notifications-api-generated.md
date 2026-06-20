## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nein | Wird verwendet, um zu bestimmen, ob die aktuelle Seite abonniert ist. |
| pageSize | integer | query | Nein |  |
| afterId | string | query | Nein |  |
| includeContext | boolean | query | Nein |  |
| afterCreatedAt | integer | query | Nein |  |
| unreadOnly | boolean | query | Nein |  |
| dmOnly | boolean | query | Nein |  |
| noDm | boolean | query | Nein |  |
| includeTranslations | boolean | query | Nein |  |
| includeTenantNotifications | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_my_notifications_response.rb)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_user_notifications'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  url_id: 'url_id_example', # String | Wird verwendet, um zu bestimmen, ob die aktuelle Seite abonniert ist.
  page_size: 56, # Integer | 
  after_id: 'after_id_example', # String | 
  include_context: true, # Boolean | 
  after_created_at: 789, # Integer | 
  unread_only: true, # Boolean | 
  dm_only: true, # Boolean | 
  no_dm: true, # Boolean | 
  include_translations: true, # Boolean | 
  include_tenant_notifications: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_notifications: #{e}"
end
[inline-code-end]