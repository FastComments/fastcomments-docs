## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| commentId | string | query | Όχι |  |
| broadcastId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/remove_user_badge_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα put_remove_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.remove_user_badge_response import RemoveUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Μπείτε σε context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργία στιγμιότυπου της κλάσης API
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (προαιρετικό)
    comment_id = 'comment_id_example' # str |  (προαιρετικό)
    broadcast_id = 'broadcast_id_example' # str |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.put_remove_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_remove_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_remove_badge: %s\n" % e)
[inline-code-end]