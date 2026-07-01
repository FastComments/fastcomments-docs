## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|-----------|------------|-----------|
| tenantId | string | Ερώτημα | Ναι |  |
| commentId | string | Διαδρομή | Ναι |  |
| broadcastId | string | Ερώτημα | Όχι |  |
| sso | string | Ερώτημα | Όχι |  |

## Response

Επιστρέφει: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/adjust_votes_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'post_adjust_comment_votes Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostAdjustCommentVotesOptions
from client.models.adjust_comment_votes_params import AdjustCommentVotesParams
from client.models.adjust_votes_response import AdjustVotesResponse
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και προεπιλεγεί στο https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισάγετε ένα context με μια παρουσία του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε μια παρουσία της κλάσης API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    adjust_comment_votes_params = client.AdjustCommentVotesParams() # AdjustCommentVotesParams | 
    broadcast_id = 'broadcast_id_example' # str |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.post_adjust_comment_votes(tenant_id, comment_id, adjust_comment_votes_params, PostAdjustCommentVotesOptions(broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->post_adjust_comment_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_adjust_comment_votes: %s\n" % e)
[inline-code-end]