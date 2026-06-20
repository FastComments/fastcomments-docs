Fasst Dokumente zusammen, indem sie gruppiert werden (falls groupBy angegeben ist) und mehrere Operationen angewendet werden.
Verschiedene Operationen (z. B. sum, countDistinct, avg, usw.) werden unterstützt.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| parentTenantId | string | query | Nein |  |
| includeStats | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## Beispiel

[inline-code-attrs-start title = 'Aggregate-Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten angegeben. Verwenden Sie das
# Beispiel, das Ihren Authentifizierungsfall erfüllt.
# API-Schlüssel-Authentifizierung konfigurieren: api_key
# Zum Aktivieren das Folgende auskommentieren, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Eine Instanz der API-Klasse erstellen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (optional)
    include_stats = True # bool |  (optional)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]