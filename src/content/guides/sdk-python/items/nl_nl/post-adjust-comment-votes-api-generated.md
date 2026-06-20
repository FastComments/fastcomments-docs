## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/adjust_votes_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'post_adjust_comment_votes Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.adjust_comment_votes_params import AdjustCommentVotesParams
from client.models.adjust_votes_response import AdjustVotesResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    adjust_comment_votes_params = client.AdjustCommentVotesParams() # AdjustCommentVotesParams | 
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.post_adjust_comment_votes(comment_id, adjust_comment_votes_params, sso=sso)
        print("The response of ModerationApi->post_adjust_comment_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_adjust_comment_votes: %s\n" % e)
[inline-code-end]

---