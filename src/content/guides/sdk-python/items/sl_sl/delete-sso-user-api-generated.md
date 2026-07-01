## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## Odziv

Vrne: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## Primer

[inline-code-attrs-start title = 'delete_sso_user Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteSsoUserOptions
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Glej configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
# Odjemalec mora konfigurirati parametre za avtentikacijo in avtorizacijo
# v skladu s varnostno politiko strežnika API.
# Primeri za vsako metodo avtentikacije so prikazani spodaj; uporabite primer,
# ki ustreza vašemu primeru uporabe avtentikacije.

# Konfiguriraj avtentikacijo s ključem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentiraj spodaj, da nastavite predpono (npr. Bearer) za ključ API, po potrebi
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopi v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvari instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (neobvezno)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (neobvezno)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, DeleteSsoUserOptions(delete_comments=delete_comments, comment_delete_mode=comment_delete_mode))
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]