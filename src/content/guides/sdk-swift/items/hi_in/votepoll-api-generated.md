## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| commentId | string | path | हाँ |  |
| urlId | string | query | नहीं |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`VotePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VotePollResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'votePoll उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let votePollBody = VotePollBody(optionId: "optionId_example") // VotePollBody | 
let urlId = "urlId_example" // String |  (optional)
let broadcastId = "broadcastId_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.votePoll(tenantId: tenantId, commentId: commentId, votePollBody: votePollBody, options: PublicAPI.VotePollOptions(urlId: urlId, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]