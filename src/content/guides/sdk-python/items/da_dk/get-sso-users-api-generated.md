## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | integer | query | Nej |  |

## Svar

Returnerer: [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_users200_response.py)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på get_sso_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_users200_response import GetSSOUsers200Response
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
# Klienten skal konfigurere autentificerings- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er vist nedenfor, brug det eksempel som
# passer til dit auth-brugstilfælde.
# Konfigurer API-nøgleautorisation: api_key
# Fjern kommentaren nedenfor for at sætte et præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 56 # int |  (valgfrit)

    try:
        api_response = api_instance.get_sso_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_sso_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_users: %s\n" % e)
[inline-code-end]