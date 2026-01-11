## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| pageSize | integer | query | Nee |  |
| afterId | string | query | Nee |  |
| includeContext | boolean | query | Nee |  |
| afterCreatedAt | integer | query | Nee |  |
| unreadOnly | boolean | query | Nee |  |
| dmOnly | boolean | query | Nee |  |
| noDm | boolean | query | Nee |  |
| includeTranslations | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Retourneert: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_notifications200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_user_notifications Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  page_size: 56, # Integer | 
  after_id: 'after_id_example', # String | 
  include_context: true, # Boolean | 
  after_created_at: 789, # Integer | 
  unread_only: true, # Boolean | 
  dm_only: true, # Boolean | 
  no_dm: true, # Boolean | 
  include_translations: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_notifications: #{e}"
end
[inline-code-end]

---