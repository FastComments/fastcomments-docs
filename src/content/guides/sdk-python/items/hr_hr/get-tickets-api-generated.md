## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Odgovor

Vraća: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_tickets'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTicketsOptions
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je optionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentifikacije prikazani su u nastavku, upotrijebite primjer koji
# zadovoljava vaš slučaj korištenja autentifikacije.

# Konfigurirajte autorizaciju API ključa: api_key
# Odkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Stvorite instancu klase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    user_id = 'user_id_example' # str |  (neobavezno)
    state = 3.4 # float |  (neobavezno)
    skip = 3.4 # float |  (neobavezno)
    limit = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_tickets(tenant_id, GetTicketsOptions(user_id=user_id, state=state, skip=skip, limit=limit))
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]