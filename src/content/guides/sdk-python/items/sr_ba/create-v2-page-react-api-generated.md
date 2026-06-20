---
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| id | string | query | Yes |  |
| title | string | query | No |  |

## Odgovor

Vraća: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_v1_page_react.py)

## Primjer

[inline-code-attrs-start title = 'Primjer create_v2_page_react'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_v1_page_react import CreateV1PageReact
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i prema zadanim postavkama je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 
    title = 'title_example' # str |  (neobavezno)

    try:
        api_response = api_instance.create_v2_page_react(tenant_id, url_id, id, title=title)
        print("The response of PublicApi->create_v2_page_react:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_v2_page_react: %s\n" % e)
[inline-code-end]

---