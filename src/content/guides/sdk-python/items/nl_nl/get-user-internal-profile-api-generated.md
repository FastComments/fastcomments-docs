---
## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| commentId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_internal_profile_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_user_internal_profile Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_internal_profile_response import GetUserInternalProfileResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.get_user_internal_profile(comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_user_internal_profile:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_internal_profile: %s\n" % e)
[inline-code-end]

---