Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτήν τη στιγμή online. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε αυτό αφού εξαντληθεί το /users/online για να εμφανίσετε μια ενότητα «Μέλη».
Σελιδοποίηση με δρομέα στο commenterName: ο server διασχίζει τον μερικό δείκτη {tenantId, urlId, commenterName} από afterName προς τα εμπρός μέσω του $gt, χωρίς κόστος $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον server). |
| afterName | string | query | No | Δρομέας: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Διαιρέτης ισοπαλίας δρομέα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοπαλίες ονομάτων να μην αποκλείουν εγγραφές. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Example

[inline-code-attrs-start title = 'Παράδειγμα get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον server).
opts = {
  after_name: 'after_name_example', # String | Δρομέας: περάστε το nextAfterName από την προηγούμενη απάντηση.
  after_user_id: 'after_user_id_example' # String | Διαιρέτης ισοπαλίας δρομέα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοπαλίες ονομάτων να μην αποκλείουν εγγραφές.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]