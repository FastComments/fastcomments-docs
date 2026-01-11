## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| updateComments | boolean | query | Ne |  |

## Odgovor

Vraća: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_sso_user_api_response.py)

## Primer

[inline-code-attrs-start title = 'Primer za patch_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_sso_user_api_response import PatchSSOUserAPIResponse
from client.models.update_apisso_user_data import UpdateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa bezbednosnom politikom API servera.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da biste podesili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_apisso_user_data = client.UpdateAPISSOUserData() # UpdateAPISSOUserData | 
    update_comments = True # bool |  (neobavezno)

    try:
        api_response = api_instance.patch_sso_user(tenant_id, id, update_apisso_user_data, update_comments=update_comments)
        print("The response of DefaultApi->patch_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_sso_user: %s\n" % e)
[inline-code-end]