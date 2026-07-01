## Parameters

| Name | Type | Location | Required | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## Respons

Retourneert: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_internal_profile_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld get_user_internal_profile'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetUserInternalProfileOptions
from client.models.get_user_internal_profile_response import GetUserInternalProfileResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een exemplaar van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een exemplaar van de API-klasse
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.get_user_internal_profile(tenant_id, GetUserInternalProfileOptions(comment_id=comment_id, sso=sso))
        print("The response of ModerationApi->get_user_internal_profile:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_internal_profile: %s\n" % e)
[inline-code-end]