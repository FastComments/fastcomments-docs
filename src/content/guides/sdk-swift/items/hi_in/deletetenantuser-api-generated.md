## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| deleteComments | string | query | नहीं |  |
| commentDeleteMode | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'deleteTenantUser उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let deleteComments = "deleteComments_example" // String |  (वैकल्पिक)
let commentDeleteMode = "commentDeleteMode_example" // String |  (वैकल्पिक)

DefaultAPI.deleteTenantUser(tenantId: tenantId, id: id, options: DefaultAPI.DeleteTenantUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]