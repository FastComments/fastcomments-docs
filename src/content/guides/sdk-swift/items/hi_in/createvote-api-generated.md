## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | query | हाँ |  |
| direction | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'createVote उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let direction = "direction_example" // String | 
let userId = "userId_example" // String |  (optional)
let anonUserId = "anonUserId_example" // String |  (optional)

DefaultAPI.createVote(tenantId: tenantId, commentId: commentId, direction: direction, options: DefaultAPI.CreateVoteOptions(userId: userId, anonUserId: anonUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]