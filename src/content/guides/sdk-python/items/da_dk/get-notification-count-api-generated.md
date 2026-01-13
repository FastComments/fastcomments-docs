## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nej |  |
| urlId | string | query | Nej |  |
| fromCommentId | string | query | Nej |  |
| viewed | boolean | query | Nej |  |
| type | string | query | Nej |  |

## Svar

Returnerer: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count200_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_notification_count Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notification_count200_response import GetNotificationCount200Response
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at definere host, standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentifikations- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er vist nedenfor; brug det eksempel,
# der passer til dit brugstilfælde.

# Konfigurer API-nøgleautorisering: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at konfigurere præfiks (f.eks. Bearer) for API-nøglen, hvis det er nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (valgfri)
    url_id = 'url_id_example' # str |  (valgfri)
    from_comment_id = 'from_comment_id_example' # str |  (valgfri)
    viewed = True # bool |  (valgfri)
    type = 'type_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_notification_count(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type)
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]