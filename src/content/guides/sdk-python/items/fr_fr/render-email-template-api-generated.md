## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| locale | string | query | Non |  |

## Réponse

Renvoie: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/render_email_template_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de render_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.render_email_template_body import RenderEmailTemplateBody
from client.models.render_email_template_response import RenderEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    render_email_template_body = client.RenderEmailTemplateBody() # RenderEmailTemplateBody | 
    locale = 'locale_example' # str |  (optional)

    try:
        api_response = api_instance.render_email_template(tenant_id, render_email_template_body, locale=locale)
        print("The response of DefaultApi->render_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->render_email_template: %s\n" % e)
[inline-code-end]