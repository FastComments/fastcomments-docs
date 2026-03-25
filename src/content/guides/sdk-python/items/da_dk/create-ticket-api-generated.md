## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Ja |  |

## Svar

Returnerer: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_ticket200_response.py)

## Eksempel

[inline-code-attrs-start title = 'create_ticket Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_ticket200_response import CreateTicket200Response
from client.models.create_ticket_body import CreateTicketBody
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at definere host, standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificerings- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver autentificeringsmetode er vist nedenfor. Brug det eksempel, der
# passer til dit autentificeringsbrugstilfælde.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    create_ticket_body = client.CreateTicketBody() # CreateTicketBody | 

    try:
        api_response = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
        print("The response of DefaultApi->create_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_ticket: %s\n" % e)
[inline-code-end]

---