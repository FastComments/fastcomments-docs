## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| namespace | string | Pfad | Ja |  |
| component | string | Pfad | Ja |  |
| locale | string | Abfrage | Nein |  |
| useFullTranslationIds | boolean | Abfrage | Nein |  |

## Antwort

Gibt zurück: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_translations Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Erstelle eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (optional)
    use_full_translation_ids = True # bool |  (optional)

    try:
        api_response = api_instance.get_translations(namespace, component, locale=locale, use_full_translation_ids=use_full_translation_ids)
        print("The response of PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_translations: %s\n" % e)
[inline-code-end]