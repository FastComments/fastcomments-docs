## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| pageSize | integer | query | Non |  |
| afterId | string | query | Non |  |
| includeContext | boolean | query | Non |  |
| afterCreatedAt | integer | query | Non |  |
| unreadOnly | boolean | query | Non |  |
| dmOnly | boolean | query | Non |  |
| noDm | boolean | query | Non |  |
| includeTranslations | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_notifications200_response.rb)

## Exemple

[inline-code-attrs-start title = 'get_user_notifications Exemple'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
opts = {
  page_size: 56, # Entier | 
  after_id: 'after_id_example', # Chaîne | 
  include_context: true, # Booléen | 
  after_created_at: 789, # Entier | 
  unread_only: true, # Booléen | 
  dm_only: true, # Booléen | 
  no_dm: true, # Booléen | 
  include_translations: true, # Booléen | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.get_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_notifications: #{e}"
end
[inline-code-end]