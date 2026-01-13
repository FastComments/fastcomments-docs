## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nej |  |
| badgeId | string | query | Nej |  |
| type | number | query | Nej |  |
| displayedOnComments | boolean | query | Nej |  |
| limit | number | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badges200_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_user_badges Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badges200_response import GetUserBadges200Response
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificering og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver godkendelsesmetode er vist nedenfor, brug det eksempel som
# opfylder dit godkendelsesscenarie.

# Konfigurer API-nøgleautorisering: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommenteringen nedenfor for at sætte præfiks (fx Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | (valgfri)
    badge_id = 'badge_id_example' # str | (valgfri)
    type = 3.4 # float | (valgfri)
    displayed_on_comments = True # bool | (valgfri)
    limit = 3.4 # float | (valgfri)
    skip = 3.4 # float | (valgfri)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]