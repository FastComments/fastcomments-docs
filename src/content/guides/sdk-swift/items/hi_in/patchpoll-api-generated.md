पोल को उसकी गिनती बनाए रखते हुए उसी स्थान पर संपादित करें: प्रश्न बदलें, किसी विकल्प का लेबल बदलें, उसे बंद या फिर से खोलें,
या बदलें कि मतदाताओं को कौन देख सकता है। विकल्प id द्वारा संबोधित किए जाते हैं - उन्हें जोड़ने, हटाने या पुनर्व्यवस्थित करने के लिए, पूरी सूची PUT करें।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'patchPoll उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPatch = CommentPollPatch(question: "question_example", options: [CommentPollPatch_options_inner(label: "label_example", id: "id_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPatch | 

DefaultAPI.patchPoll(tenantId: tenantId, commentId: commentId, commentPollPatch: commentPollPatch) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]