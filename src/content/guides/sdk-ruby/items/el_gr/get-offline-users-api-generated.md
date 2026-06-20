---
Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτήν τη στιγμή online. Ταξινομημένα κατά displayName.
Χρησιμοποιήστε αυτό αφού εξαντλήσετε το /users/online για να αποδώσετε μια ενότητα "Μέλη".
Σελιδοποίηση cursor στο commenterName: ο διακομιστής διασχίζει τον μερικό δείκτη {tenantId, urlId, commenterName}
Δείκτης από afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | query | Όχι | Δείκτης ισοπαλίας: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν το afterName είναι ρυθμισμένο ώστε οι ισοβαθμίες ονομάτων να μην απορρίψουν καταχωρήσεις. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή).
opts = {
  after_name: 'after_name_example', # String | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση.
  after_user_id: 'after_user_id_example' # String | Δείκτης ισοπαλίας (tiebreaker): περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοβαθμίες ονομάτων να μην απορρίψουν καταχωρήσεις.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---