## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| redirectURL | string | query | Ne |  |

## Response

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer za send_login_link'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa politikom bezbednosti API servera.
# Primeri za svaki metod autentifikacije su dati ispod, upotrebite primer koji
# odgovara vašem slučaju upotrebe.
# Konfigurišite autorizaciju API ključa: api_key
# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    redirect_url = 'redirect_url_example' # str |  (neobavezno)

    try:
        api_response = api_instance.send_login_link(tenant_id, id, redirect_url=redirect_url)
        print("The response of DefaultApi->send_login_link:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->send_login_link: %s\n" % e)
[inline-code-end]