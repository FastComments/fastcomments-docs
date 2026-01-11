---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| updateComments | boolean | query | Ne |  |

## Odgovor

Vraća: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_sso_user_api_response.py)

## Primjer

[inline-code-attrs-start title = 'patch_sso_user Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_sso_user_api_response import PatchSSOUserAPIResponse
from client.models.update_apisso_user_data import UpdateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i prema zadanim postavkama koristi https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentikacije navedeni su u nastavku, upotrijebite onaj
# koji odgovara vašem slučaju uporabe autentikacije.

# Konfigurirajte autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ako je potrebno, uklonite komentar ispod za postavljanje prefiksa (npr. Bearer) za API ključ
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
        api_response = api_instance.patch_sso_user(tenant_id, id, update_apisso_user_data, update_comments=update_comments)
        print("The response of DefaultApi->patch_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_sso_user: %s\n" % e)
[inline-code-end]

---