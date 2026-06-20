## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| badgeId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| commentId | string | query | Όχι |  |
| broadcastId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/award_user_badge_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα put_award_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.award_user_badge_response import AwardUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένα χρησιμοποιείται το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ανοίξτε ένα context με ένα στιγμιότυπο του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (προαιρετικό)
    comment_id = 'comment_id_example' # str |  (προαιρετικό)
    broadcast_id = 'broadcast_id_example' # str |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.put_award_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_award_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_award_badge: %s\n" % e)
[inline-code-end]