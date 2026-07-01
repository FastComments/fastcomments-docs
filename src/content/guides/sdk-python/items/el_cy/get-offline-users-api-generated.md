Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή). |
| afterName | string | query | No | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | query | No | Διαχωριστικό κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν afterName έχει οριστεί ώστε τα ισοδύναμα ονόματα να μην αφαιρεθούν. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'Παράδειγμα get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και προεπιλεγμένος στο https://fastcomments.com
# Δείτε το configuration.py για τη λίστα όλων των υποστηριζόμενων παραμέτρων διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή).
    after_name = 'after_name_example' # str | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απόκριση. (προαιρετικό)
    after_user_id = 'after_user_id_example' # str | Διαχωριστικό κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν afterName έχει οριστεί ώστε τα ισοδύναμα ονόματα να μην αφαιρεθούν. (προαιρετικό)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]