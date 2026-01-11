## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| updateComments | boolean | query | Ne |  |

## Odgovor

Vraća: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/put_sso_user_api_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer put_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.put_sso_user_api_response import PutSSOUserAPIResponse
from client.models.update_apisso_user_data import UpdateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# Postavljanje hosta je opcionalno i zadano je na https://fastcomments.com
# Vidi configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaki način autentifikacije su navedeni dolje, upotrijebite primjer koji
# odgovara vašem slučaju korištenja autentifikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Stvorite instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_apisso_user_data = client.UpdateAPISSOUserData() # UpdateAPISSOUserData | 
    update_comments = True # bool |  (neobavezno)

    try:
        api_response = api_instance.put_sso_user(tenant_id, id, update_apisso_user_data, update_comments=update_comments)
        print("The response of DefaultApi->put_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_sso_user: %s\n" % e)
[inline-code-end]