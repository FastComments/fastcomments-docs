पोल को उसके कमेंट से हटाएँ, साथ ही उस पर डाले गए हर वोट को भी। कमेंट स्वयं अछूता रहता है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'deletePoll उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 

DefaultAPI.deletePoll(tenantId: tenantId, commentId: commentId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]