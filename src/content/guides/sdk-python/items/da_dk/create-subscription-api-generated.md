## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_subscription_api_response.py)

## Eksempel

[inline-code-attrs-start title = 'create_subscription Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_api_user_subscription_data import CreateAPIUserSubscriptionData
from client.models.create_subscription_api_response import CreateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# At definere host er valgfrit og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentifikations- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er vist nedenfor; brug det eksempel der
# opfylder dit auth-brugsscenarie.

# Konfigurer API-nøgle-autorisering: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentar nedenfor for at indstille præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_user_subscription_data = client.CreateAPIUserSubscriptionData() # CreateAPIUserSubscriptionData | 

    try:
        api_response = api_instance.create_subscription(tenant_id, create_api_user_subscription_data)
        print("The response of DefaultApi->create_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_subscription: %s\n" % e)
[inline-code-end]