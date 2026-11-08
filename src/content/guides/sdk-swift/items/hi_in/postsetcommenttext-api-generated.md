## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| broadcastId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentTextResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'postSetCommentText उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new के माध्यम से रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let setCommentTextParams = SetCommentTextParams(comment: "comment_example") // SetCommentTextParams | 
let broadcastId = "broadcastId_example" // String |  (वैकल्पिक)
let sso = "sso_example" // String |  (वैकल्पिक)

ModerationAPI.postSetCommentText(tenantId: tenantId, commentId: commentId, setCommentTextParams: setCommentTextParams, options: ModerationAPI.PostSetCommentTextOptions(broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]