## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIModerateGetUserBanPreferencesResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getUserBanPreference'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još uvek u beta fazi. Za bilo koji problem, molimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let sso = "sso_example" // String |  (neobavezno)

ModerationAPI.getUserBanPreference(sso: sso) { (response, error) in
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