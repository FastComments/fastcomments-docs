## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| direction | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'post_vote Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένο είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Χρησιμοποιήστε ένα πλαίσιο με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_vote(comment_id, direction=direction, sso=sso)
        print("The response of ModerationApi->post_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_vote: %s\n" % e)
[inline-code-end]