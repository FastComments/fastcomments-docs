## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| commentId | string | pad | Ja |  |
| direction | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'post_vote Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.post_vote(comment_id, direction=direction, sso=sso)
        print("The response of ModerationApi->post_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_vote: %s\n" % e)
[inline-code-end]