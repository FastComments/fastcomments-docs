## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_templates_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_email_templates Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_templates_response import GetEmailTemplatesResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og som standard https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificerings- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver godkendelsesmetode er vist nedenfor, brug det eksempel, der
# passer til dit godkendelsesscenarie.

# Konfigurer API-nøgle-autorisering: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommenteringen nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (valgfri)

    try:
        api_response = api_instance.get_email_templates(tenant_id, skip=skip)
        print("The response of DefaultApi->get_email_templates:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_templates: %s\n" % e)
[inline-code-end]