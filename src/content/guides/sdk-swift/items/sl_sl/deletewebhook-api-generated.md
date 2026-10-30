Odjave (REST hook odjava). Le naročnine, ustvarjene prek tega API-ja, je mogoče izbrisati tukaj; spletne kljuke, upravljane prek nadzorne plošče, se urejajo v nadzorni plošči.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odziv

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Primer

[inline-code-attrs-start title = 'deleteWebhook Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. V primeru težav, prosimo, poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.deleteWebhook(tenantId: tenantId, id: id) { (response, error) in
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