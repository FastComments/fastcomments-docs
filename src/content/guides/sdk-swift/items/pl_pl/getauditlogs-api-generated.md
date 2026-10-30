## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Tenant, dla którego odczytywane są logi audytu. |
| limit | integer | query | No | Ile logów zwrócić. Maksymalnie 10000, domyślnie 1000. |
| skip | integer | query | No | Ile logów pominąć, w celu paginacji. |
| order | string | query | No | Kierunek sortowania według czasu zdarzenia. Domyślnie DESC (najnowsze najpierw). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Zwróć tylko zdarzenia wykonane przez tę nazwę użytkownika. |
| ip | string | query | No | Zwróć tylko zdarzenia z tego adresu IP. |
| crudType | string | query | No | Zwróć tylko zdarzenia tego typu: c, r, u, d lub login. |
| resourceName | string | query | No | Zwróć tylko zdarzenia dla tego zasobu, np. User lub Moderator. |
| targetId | string | query | No | Zwróć tylko zdarzenia, których dotknięty obiekt ma ten identyfikator. |
| target | string | query | No | Dopasowanie podciągu (bez rozróżniania wielkości liter) w etykiecie dotkniętego obiektu, np. nazwa użytkownika lub e-mail. |
| includeManagedTenants | boolean | query | No | Zwróć także zdarzenia z tenantów, które ten tenant zarządza. |

## Odpowiedź

Zwraca: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W przypadku jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Tenant, dla którego odczytywane są logi audytu.
let limit = 987 // Int | Ile logów zwrócić. Maksymalnie 10000, domyślnie 1000. (opcjonalnie)
let skip = 987 // Int | Ile logów pominąć, w celu paginacji. (opcjonalnie)
let order = SORT_DIR() // SORTDIR | Kierunek sortowania według czasu zdarzenia. Domyślnie DESC (najnowsze najpierw). (opcjonalnie)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Zwróć tylko zdarzenia wykonane przez tę nazwę użytkownika. (opcjonalnie)
let ip = "ip_example" // String | Zwróć tylko zdarzenia z tego adresu IP. (opcjonalnie)
let crudType = "crudType_example" // String | Zwróć tylko zdarzenia tego typu: c, r, u, d lub login. (opcjonalnie)
let resourceName = "resourceName_example" // String | Zwróć tylko zdarzenia dla tego zasobu, np. User lub Moderator. (opcjonalnie)
let targetId = "targetId_example" // String | Zwróć tylko zdarzenia, których dotknięty obiekt ma ten identyfikator. (opcjonalnie)
let target = "target_example" // String | Dopasowanie podciągu (bez rozróżniania wielkości liter) w etykiecie dotkniętego obiektu, np. nazwa użytkownika lub e-mail. (opcjonalnie)
let includeManagedTenants = true // Bool | Zwróć także zdarzenia z tenantów, które ten tenant zarządza. (opcjonalnie)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]