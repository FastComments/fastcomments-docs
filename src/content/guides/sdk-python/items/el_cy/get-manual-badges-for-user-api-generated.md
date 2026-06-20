## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Όχι |  |
| commentId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_manual_badges_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_manual_badges_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_manual_badges_response import GetUserManualBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και προεπιλεγμένη τιμή είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Μπείτε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.ModerationApi(api_client)
    badges_user_id = 'badges_user_id_example' # str |  (προαιρετικό)
    comment_id = 'comment_id_example' # str |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.get_manual_badges_for_user(badges_user_id=badges_user_id, comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_manual_badges_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_manual_badges_for_user: %s\n" % e)
[inline-code-end]