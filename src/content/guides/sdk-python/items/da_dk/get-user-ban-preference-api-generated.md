## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_moderate_get_user_ban_preferences_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_user_ban_preference Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_moderate_get_user_ban_preferences_response import APIModerateGetUserBanPreferencesResponse
from client.rest import ApiException
from pprint import pprint

# Definerer værten er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Indtast en kontekst med en forekomst af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en forekomst af API-klassen
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_user_ban_preference(tenant_id, sso=sso)
        print("The response of ModerationApi->get_user_ban_preference:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_ban_preference: %s\n" % e)
[inline-code-end]