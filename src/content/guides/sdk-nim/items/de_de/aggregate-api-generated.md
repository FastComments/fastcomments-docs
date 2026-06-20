Fasst Dokumente zusammen, indem sie gruppiert werden (falls groupBy angegeben ist) und mehrere Operationen angewendet werden. Verschiedene Operationen (z. B. sum, countDistinct, avg usw.) werden unterstützt.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Nein |  |
| parentTenantId | string | Nein |  |
| includeStats | bool | Nein |  |

## Antwort

Gibt zurück: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Beispiel

[inline-code-attrs-start title = 'aggregate Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregate(
  tenantId = "my-tenant-123",
  aggregationRequest = AggregationRequest(groupBy = @["articleId"], metrics = @["commentCount"], filters = @[], limit = 0),
  parentTenantId = "",
  includeStats = false
)

if response.isSome:
  let agg = response.get()
  discard agg
[inline-code-end]

---