## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_text_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα post_set_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_text_params import SetCommentTextParams
from client.models.set_comment_text_response import SetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    set_comment_text_params = client.SetCommentTextParams() # SetCommentTextParams | 
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.post_set_comment_text(comment_id, set_comment_text_params, sso=sso)
        print("The response of ModerationApi->post_set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_text: %s\n" % e)
[inline-code-end]