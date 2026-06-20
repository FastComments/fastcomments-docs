## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Όχι | Χρησιμοποιείται για να προσδιοριστεί αν η τρέχουσα σελίδα είναι εγγεγραμμένη. |
| pageSize | integer | query | Όχι |  |
| afterId | string | query | Όχι |  |
| includeContext | boolean | query | Όχι |  |
| afterCreatedAt | integer | query | Όχι |  |
| unreadOnly | boolean | query | Όχι |  |
| dmOnly | boolean | query | Όχι |  |
| noDm | boolean | query | Όχι |  |
| includeTranslations | boolean | query | Όχι |  |
| includeTenantNotifications | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_my_notifications_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_user_notifications'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  url_id: 'url_id_example', # String | Χρησιμοποιείται για να προσδιοριστεί αν η τρέχουσα σελίδα είναι εγγεγραμμένη.
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

---