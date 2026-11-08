## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| commentId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| broadcastId | string | query | हाँ |  |
| sessionId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'voteComment उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let voteBodyParams = VoteBodyParams(commenterEmail: "commenterEmail_example", commenterName: "commenterName_example", voteDir: "voteDir_example", url: "url_example") // VoteBodyParams | 
let sessionId = "sessionId_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.voteComment(tenantId: tenantId, commentId: commentId, urlId: urlId, broadcastId: broadcastId, voteBodyParams: voteBodyParams, options: PublicAPI.VoteCommentOptions(sessionId: sessionId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]