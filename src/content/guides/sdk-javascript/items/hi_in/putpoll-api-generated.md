किसी कमेंट से एक पोल जोड़ें, या उसके पास पहले से मौजूद पोल की पूरी स्थिति सेट करें।

विकल्प id द्वारा मैच किए जाते हैं: किसी मौजूदा विकल्प की id के साथ भेजा गया विकल्प अपने वोट बरकरार रखता है (और नया लेबल तथा स्थान ले लेता है), बिना id भेजा गया विकल्प जोड़ दिया जाता है, और सूची से बाहर छूटे हुए मौजूदा विकल्प उन पर डाले गए वोटों सहित हटा दिए जाते हैं।

वोट वाले पोल पर कोई भी मौजूदा विकल्प id न रखने पर उन सभी को हटा दिया जाता है, इसलिए इसके लिए replaceVotes=true की आवश्यकता होती है।

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'putPoll उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const commentId: string = "9876543210";

const optionA: CommentPollOptionInput = { text: "Dark mode" };
const optionB: CommentPollOptionInput = { text: "Light mode" };

const pollInput: CommentPollPutInput = {
  question: "Which UI theme do you prefer?",
  options: [optionA, optionB],
};

const replaceVotes: boolean = true;

const result: SavePollResponse = await putPoll(tenantId, commentId, pollInput, replaceVotes);
[inline-code-end]