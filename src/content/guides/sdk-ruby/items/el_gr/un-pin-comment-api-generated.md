## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| commentId | string | path | Ναι |  |
| broadcastId | string | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/change_comment_pin_status_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'un_pin_comment Παράδειγμα'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Συμβολοσειρά | 
comment_id = 'comment_id_example' # Συμβολοσειρά | 
broadcast_id = 'broadcast_id_example' # Συμβολοσειρά | 
opts = {
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.un_pin_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->un_pin_comment: #{e}"
end
[inline-code-end]

---