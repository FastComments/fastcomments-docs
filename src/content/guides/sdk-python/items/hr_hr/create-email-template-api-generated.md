## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_email_template_response.py)

## Primjer

[inline-code-attrs-start title = 'create_email_template Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_email_template_body import CreateEmailTemplateBody
from client.models.create_email_template_response import CreateEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# Postavljanje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# sukladno sigurnosnoj politici API servera.
# Primjeri za svaku metodu autentikacije navedeni su u nastavku, upotrijebite primjer koji
# odgovara vašem scenariju autentikacije.

# Konfigurirajte autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje da biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
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