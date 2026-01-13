---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| yearNumber | number | query | Nie |  |
| monthNumber | number | query | Nie |  |
| dayNumber | number | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsages200Response.swift)

## Przykład

[inline-code-attrs-start title = 'getTenantDailyUsages Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemów zgłoś je poprzez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (opcjonalne)
let monthNumber = 987 // Double |  (opcjonalne)
let dayNumber = 987 // Double |  (opcjonalne)
let skip = 987 // Double |  (opcjonalne)

DefaultAPI.getTenantDailyUsages(tenantId: tenantId, yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---