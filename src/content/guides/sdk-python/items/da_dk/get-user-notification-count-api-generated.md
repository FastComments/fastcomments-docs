## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_notification_count_response.py)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på get_user_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_notification_count_response import GetUserNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_user_notification_count(tenant_id, sso=sso)
        print("The response of PublicApi->get_user_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notification_count: %s\n" % e)
[inline-code-end]

---