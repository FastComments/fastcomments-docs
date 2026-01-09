Aggrega i documenti raggruppandoli (se groupBy è fornito) e applicando più operazioni. Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| aggregationRequest | AggregationRequest | Sì |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Response

Restituisce: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---