## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postId | string | path | हाँ |  |
| isUndo | boolean | query | नहीं |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ReactFeedPostResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'reactFeedPostPublic उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postId = "postId_example" // String | 
let reactBodyParams = ReactBodyParams(reactType: "reactType_example") // ReactBodyParams | 
let isUndo = true // Bool |  (optional)
let broadcastId = "broadcastId_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.reactFeedPostPublic(tenantId: tenantId, postId: postId, reactBodyParams: reactBodyParams, options: PublicAPI.ReactFeedPostPublicOptions(isUndo: isUndo, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]