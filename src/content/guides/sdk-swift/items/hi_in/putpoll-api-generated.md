किसी कमेंट से एक पोल जोड़ें, या उसके पास पहले से मौजूद पोल की पूरी स्थिति सेट करें।

विकल्प id द्वारा मैच किए जाते हैं: किसी मौजूदा विकल्प की id के साथ भेजा गया विकल्प अपने वोट बरकरार रखता है (और नया लेबल तथा स्थान ले लेता है), बिना id वाला विकल्प जोड़ दिया जाता है, और सूची से बाहर छूटे मौजूदा विकल्प उन पर डाले गए वोटों सहित हटा दिए जाते हैं।

वोट वाले पोल पर कोई भी मौजूदा विकल्प id न रखने से उन सभी को हटा दिया जाता है, इसलिए इसके लिए replaceVotes=true की आवश्यकता होती है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| replaceVotes | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'putPoll उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (optional)

DefaultAPI.putPoll(tenantId: tenantId, commentId: commentId, commentPollPutInput: commentPollPutInput, replaceVotes: replaceVotes) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]