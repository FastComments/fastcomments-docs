Aggregeert documenten door ze te groeperen (als groupBy wordt opgegeven) en meerdere bewerkingen toe te passen. Verschillende bewerkingen (bijv. sum, countDistinct, avg, etc.) worden ondersteund.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Ja |  |
| parentTenantId | string | Nee |  |
| includeStats | boolean | Nee |  |

## Antwoord

Retourneert: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---