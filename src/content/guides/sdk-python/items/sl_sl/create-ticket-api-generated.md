## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Da |  |

## Odgovor

Vrne: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_ticket200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer create_ticket'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_ticket200_response import CreateTicket200Response
from client.models.create_ticket_body import CreateTicketBody
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre za overjanje in avtorizacijo
# v skladu s varnostno politiko API strežnika.
# Spodaj so prikazani primeri za vsak način overjanja, uporabite primer, ki
# ustreza vašemu primeru uporabe overjanja.

# Konfigurirajte overjanje z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnje, če potrebujete nastavitev predpone (npr. Bearer) za API ključ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
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