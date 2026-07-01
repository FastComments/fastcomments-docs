## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-----------|
| tenantId | string | ερώτημα | Ναι |  |
| commentId | string | διαδρομή | Ναι |  |
| includeByUserIdAndEmail | boolean | ερώτημα | Όχι |  |
| includeByIP | boolean | ερώτημα | Όχι |  |
| includeByEmailDomain | boolean | ερώτημα | Όχι |  |
| sso | string | ερώτημα | Όχι |  |

## Απόκριση

Επιστρέφει: [`PreBanSummary`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/pre_ban_summary.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'get_pre_ban_summary Παράδειγμα'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
opts = {
  include_by_user_id_and_email: true, # Boolean | 
  include_by_ip: true, # Boolean | 
  include_by_email_domain: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_pre_ban_summary(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_pre_ban_summary: #{e}"
end
[inline-code-end]