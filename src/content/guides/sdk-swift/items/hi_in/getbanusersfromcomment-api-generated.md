## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersFromCommentResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getBanUsersFromComment उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (optional)

ModerationAPI.getBanUsersFromComment(tenantId: tenantId, commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]