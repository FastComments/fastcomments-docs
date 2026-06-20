## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer uporabe logoutPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Če naletite na težavo, jo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
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