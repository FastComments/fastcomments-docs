## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vrača: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_ticket_state200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer change_ticket_state'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_ticket_state200_response import ChangeTicketState200Response
from client.models.change_ticket_state_body import ChangeTicketStateBody
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Za seznam vseh podprtih konfiguracijskih parametrov glejte configuration.py.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre overjanja in avtorizacije
# v skladu z varnostno politiko strežnika API.
# Primeri za vsak način overjanja so podani spodaj, uporabite tisti primer,
# ki ustreza vašemu primeru uporabe overjanja.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    id = 'id_example' # str | 
    change_ticket_state_body = client.ChangeTicketStateBody() # ChangeTicketStateBody | 

    try:
        api_response = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
        print("The response of DefaultApi->change_ticket_state:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->change_ticket_state: %s\n" % e)
[inline-code-end]