## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di logoutPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift


PublicAPI.logoutPublic() { (response, error) in
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