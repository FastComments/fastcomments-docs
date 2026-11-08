किस विकल्प पर किसने वोट दिया। गुमनाम पोल कभी जवाब नहीं देते, केवल-एडमिन पोल केवल एडमिन या मॉडरेटर के लिए जवाब देते हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| commentId | string | path | हाँ |  |
| urlId | string | query | नहीं |  |
| page | integer | query | नहीं |  |
| search | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getPollVoters उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (optional)
let page = 987 // Int |  (optional)
let search = "search_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.getPollVoters(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetPollVotersOptions(urlId: urlId, page: page, search: search, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]