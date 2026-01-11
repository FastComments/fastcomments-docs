## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| pageSize | integer | query | Όχι |  |
| afterId | string | query | Όχι |  |
| includeContext | boolean | query | Όχι |  |
| afterCreatedAt | integer | query | Όχι |  |
| unreadOnly | boolean | query | Όχι |  |
| dmOnly | boolean | query | Όχι |  |
| noDm | boolean | query | Όχι |  |
| includeTranslations | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_notifications200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_user_notifications'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Συμβολοσειρά | 
opts = {
  page_size: 56, # Ακέραιος | 
  after_id: 'after_id_example', # Συμβολοσειρά | 
  include_context: true, # Λογική τιμή | 
  after_created_at: 789, # Ακέραιος | 
  unread_only: true, # Λογική τιμή | 
  dm_only: true, # Λογική τιμή | 
  no_dm: true, # Λογική τιμή | 
  include_translations: true, # Λογική τιμή | 
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.get_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_notifications: #{e}"
end
[inline-code-end]