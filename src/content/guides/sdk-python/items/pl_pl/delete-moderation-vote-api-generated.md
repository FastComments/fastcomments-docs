## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| voteId | string | path | Yes |  |
| sso | string | query | No |  |

## Response

Zwraca: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład delete_moderation_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    vote_id = 'vote_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.delete_moderation_vote(comment_id, vote_id, sso=sso)
        print("The response of ModerationApi->delete_moderation_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->delete_moderation_vote: %s\n" % e)
[inline-code-end]