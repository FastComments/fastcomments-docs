## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| locale | string | query | Ne |  |

## Odgovor

Vrne: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/render_email_template_response.py)

## Primer

[inline-code-attrs-start title = 'Primer render_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.render_email_template_body import RenderEmailTemplateBody
from client.models.render_email_template_response import RenderEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in je privzeto nastavljeno na https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre overjanja in pooblastila
# v skladu z varnostno politiko API strežnika.
# Spodaj so podani primeri za vsako metodo overjanja, uporabite tistega, ki
# ustreza vašemu primeru uporabe.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, če želite nastaviti predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    render_email_template_body = client.RenderEmailTemplateBody() # RenderEmailTemplateBody | 
    locale = 'locale_example' # str |  (neobvezno)

    try:
        api_response = api_instance.render_email_template(tenant_id, render_email_template_body, locale=locale)
        print("The response of DefaultApi->render_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->render_email_template: %s\n" % e)
[inline-code-end]