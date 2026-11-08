## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplateRenderErrorsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let skip = 987 // Double |  (वैकल्पिक)

DefaultAPI.getEmailTemplateRenderErrors(tenantId: tenantId, id: id, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]