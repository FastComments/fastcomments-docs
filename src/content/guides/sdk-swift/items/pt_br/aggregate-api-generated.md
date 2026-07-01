Aggrega documentos agrupando-os (se `groupBy` for fornecido) e aplicando múltiplas operações.  
Operações diferentes (por exemplo, sum, countDistinct, avg, etc.) são suportadas.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| parentTenantId | string | query | Não |  |
| includeStats | boolean | query | Não |  |

## Resposta

Retorna: [`AggregateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de agregação'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// O seguinte exemplo de código ainda está em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let aggregationRequest = AggregationRequest(query: [QueryPredicate(key: "key_example", value: QueryPredicate_value(), _operator: "_operator_example")], resourceName: "resourceName_example", groupBy: ["groupBy_example"], operations: [AggregationOperation(field: "field_example", op: AggregationOpType(), alias: "alias_example", expandArray: false)], sort: AggregationRequest_sort(dir: "dir_example", field: "field_example")) // AggregationRequest | 
let parentTenantId = "parentTenantId_example" // String |  (optional)
let includeStats = true // Bool |  (optional)

DefaultAPI.aggregate(tenantId: tenantId, aggregationRequest: aggregationRequest, options: DefaultAPI.AggregateOptions(parentTenantId: parentTenantId, includeStats: includeStats)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]