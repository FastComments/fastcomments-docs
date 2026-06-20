## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| locale | string | query | Ne |  |

## Odgovor

Vraća: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/render_email_template_response.py)

## Primjer

[inline-code-attrs-start title = 'render_email_template Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.render_email_template_body import RenderEmailTemplateBody
from client.models.render_email_template_response import RenderEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i prema zadanim postavkama je https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaki način autentifikacije su navedeni dolje, koristite onaj primjer koji
# zadovoljava vaš slučaj upotrebe autentifikacije.

# Konfigurirajte autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uklonite komentar ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    render_email_template_body = client.RenderEmailTemplateBody() # RenderEmailTemplateBody | 
    locale = 'locale_example' # str |  (neobavezno)

    try:
        api_response = api_instance.render_email_template(tenant_id, render_email_template_body, locale=locale)
        print("The response of DefaultApi->render_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->render_email_template: %s\n" % e)
[inline-code-end]