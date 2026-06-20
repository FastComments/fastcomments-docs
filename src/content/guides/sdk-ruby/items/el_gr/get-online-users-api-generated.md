---
Τρέχοντες θεατές μιας σελίδας: άτομα των οποίων η websocket συνεδρία είναι εγγεγραμμένη στη σελίδα αυτή αυτή τη στιγμή.
Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL της σελίδας (καθαρισμένο από τον διακομιστή). |
| afterName | string | query | Όχι | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Αντιμετώπιση ισοπαλίας για τον δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε εγγραφές με ίδια ονόματα να μην παραλείπονται. |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Αναγνωριστικό URL της σελίδας (καθαρισμένο από τον διακομιστή).
opts = {
  after_name: 'after_name_example', # String | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση.
  after_user_id: 'after_user_id_example' # String | Αντιμετώπιση ισοπαλίας για τον δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε εγγραφές με ίδια ονόματα να μην παραλείπονται.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---