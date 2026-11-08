## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| banEmail | boolean | query | नहीं |  |
| banEmailDomain | boolean | query | नहीं |  |
| banIP | boolean | query | नहीं |  |
| deleteAllUsersComments | boolean | query | नहीं |  |
| bannedUntil | string | query | नहीं |  |
| isShadowBan | boolean | query | नहीं |  |
| updateId | string | query | नहीं |  |
| banReason | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## उदाहरण

[inline-code-attrs-start title = 'postBanUserFromComment उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (वैकल्पिक)
let banEmailDomain = true // Bool |  (वैकल्पिक)
let banIP = true // Bool |  (वैकल्पिक)
let deleteAllUsersComments = true // Bool |  (वैकल्पिक)
let bannedUntil = "bannedUntil_example" // String |  (वैकल्पिक)
let isShadowBan = true // Bool |  (वैकल्पिक)
let updateId = "updateId_example" // String |  (वैकल्पिक)
let banReason = "banReason_example" // String |  (वैकल्पिक)
let sso = "sso_example" // String |  (वैकल्पिक)

ModerationAPI.postBanUserFromComment(tenantId: tenantId, commentId: commentId, options: ModerationAPI.PostBanUserFromCommentOptions(banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]