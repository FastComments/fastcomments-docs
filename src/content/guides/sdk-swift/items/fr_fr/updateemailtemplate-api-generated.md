## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Renvoie: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple updateEmailTemplate'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateEmailTemplateBody = UpdateEmailTemplateBody(emailTemplateId: "emailTemplateId_example", displayName: "displayName_example", ejs: "ejs_example", domain: "domain_example", translationOverridesByLocale: "TODO", testData: "TODO") // UpdateEmailTemplateBody | 

DefaultAPI.updateEmailTemplate(tenantId: tenantId, id: id, updateEmailTemplateBody: updateEmailTemplateBody) { (response, error) in
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