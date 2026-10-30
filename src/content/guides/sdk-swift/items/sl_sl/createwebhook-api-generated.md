---
Naroči URL na dogodek komentarja (REST hook naročnina). Ponovno naročanje istega URL‑ja na isti
dogodek in domeno vrne obstoječo naročnino. Dostave so podpisane s HMAC, glejte
priročnik za webhooke; starejši `token` glavi nikoli ni poslan v API naročnine.

## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |

## Response

Vrne: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateWebhookResponse.swift)

## Example

[inline-code-attrs-start title = 'createWebhook Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za morebitne težave, prosimo, poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createWebhookParams = CreateWebhookParams(url: "url_example", event: WebhookEventName(), domain: "domain_example", method: WebhookHTTPMethod()) // CreateWebhookParams | 

DefaultAPI.createWebhook(tenantId: tenantId, createWebhookParams: createWebhookParams) { (response, error) in
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