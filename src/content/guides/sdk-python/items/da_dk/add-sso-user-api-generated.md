## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_sso_user_api_response.py)

## Eksempel

[inline-code-attrs-start title = 'add_sso_user Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_sso_user_api_response import AddSSOUserAPIResponse
from client.models.create_apisso_user_data import CreateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host; standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere godkendelses- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler på hver godkendelsesmetode er angivet nedenfor; brug det eksempel
# der opfylder dit godkendelsesscenarie.

# Konfigurer API-nøgle-godkendelse: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentar på linjen nedenfor for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_apisso_user_data = client.CreateAPISSOUserData() # CreateAPISSOUserData | 

    try:
        api_response = api_instance.add_sso_user(tenant_id, create_apisso_user_data)
        print("The response of DefaultApi->add_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_sso_user: %s\n" % e)
[inline-code-end]