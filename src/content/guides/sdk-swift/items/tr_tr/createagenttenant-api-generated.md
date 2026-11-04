İnsan kaydı olmadan bir AI ajanı için yeni bir deneme hesabı oluşturur. Bu çağrı için API anahtarı gerekmez.

Yanıt, tenant id, REST API ve MCP sunucusuna karşı hemen çalışabilen bir API anahtarı ve bir claim URL içerir. Claim URL'sini sizin için çalışan insana verin: FastComments'a giriş yapmışken açılması hesabı ona bağlar. Talep edilmemiş (unclaimed) hesaplar ve anahtarları, oluşturulduktan 72 saat sonra silinir. Talep edilene kadar hesap standart deneme limitlerine sahiptir.

## Yanıt

Döndürür: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Örnek

[inline-code-attrs-start title = 'createAgentTenant Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let createAgentTenantBody = CreateAgentTenantBody(name: "name_example", domains: ["domains_example"], agent: "agent_example") // CreateAgentTenantBody | 

DefaultAPI.createAgentTenant(createAgentTenantBody: createAgentTenantBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]