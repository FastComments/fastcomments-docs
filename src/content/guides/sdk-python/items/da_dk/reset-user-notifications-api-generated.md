## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | forespørgsel | Ja |  |
| afterId | string | forespørgsel | Nej |  |
| afterCreatedAt | integer | forespørgsel | Nej |  |
| unreadOnly | boolean | forespørgsel | Nej |  |
| dmOnly | boolean | forespørgsel | Nej |  |
| noDm | boolean | forespørgsel | Nej |  |
| sso | string | forespørgsel | Nej |  |

## Svar

Returnerer: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications_response.py)

## Eksempel

[inline-code-attrs-start title = 'reset_user_notifications Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import ResetUserNotificationsOptions
from client.models.reset_user_notifications_response import ResetUserNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og har standardværdien https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Indtast en kontekst med en forekomst af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en forekomst af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (valgfri)
    after_created_at = 56 # int |  (valgfri)
    unread_only = True # bool |  (valgfri)
    dm_only = True # bool |  (valgfri)
    no_dm = True # bool |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.reset_user_notifications(tenant_id, ResetUserNotificationsOptions(after_id=after_id, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, sso=sso))
        print("The response of PublicApi->reset_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notifications: %s\n" % e)
[inline-code-end]