## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Odgovor

Vraća: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_email_template200_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer create_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_email_template200_response import CreateEmailTemplate200Response
from client.models.create_email_template_body import CreateEmailTemplateBody
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentikacije navedeni su ispod, upotrijebite primjer koji
# odgovara vašem slučaju upotrebe autentikacije.
# Konfigurirajte autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje kako biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_email_template_body = client.CreateEmailTemplateBody() # CreateEmailTemplateBody | 

    try:
        api_response = api_instance.create_email_template(tenant_id, create_email_template_body)
        print("The response of DefaultApi->create_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_email_template: %s\n" % e)
[inline-code-end]