## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentIds | string | query | Ναι | Μια λίστα αναγνωριστικών σχολίων, χωρισμένων με κόμμα. |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/check_blocked_comments_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα checked_comments_for_blocked'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_ids = 'comment_ids_example' # String | Μια λίστα αναγνωριστικών σχολίων, χωρισμένων με κόμμα.
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->checked_comments_for_blocked: #{e}"
end
[inline-code-end]

---