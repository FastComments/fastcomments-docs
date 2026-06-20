## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_logs_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'get_logs Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_logs_response import ModerationAPIGetLogsResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα πλαίσιο με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.get_logs(comment_id, sso=sso)
        print("The response of ModerationApi->get_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_logs: %s\n" % e)
[inline-code-end]