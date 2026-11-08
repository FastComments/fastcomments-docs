[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

एक पोल की गिनती के पीछे के व्यक्तिगत वोटों की सूची देता है, सबसे पुराने पहले। प्रत्येक 100 वोट लौटाने पर एक क्रेडिट।

एक पोल एक कमेंट से संबंधित होता है, इसलिए वोट एक समय में एक पोल के लिए पढ़े जाते हैं और `commentId` आवश्यक है। इसे और सीमित करें
`voterId` से यह जांचने के लिए कि एक व्यक्ति ने कैसे वोट किया, या `optionId` से यह सूचीबद्ध करने के लिए कि किसने किसी दिए गए विकल्प को चुना।

प्रति कॉल अधिकतम 1000 वोट लौटाए जाते हैं। अधिक के लिए पेज करने के लिए `skip` का उपयोग करें।

पोल की `privacy` सेटिंग का सम्मान किया जाता है: गुमनाम पोल पर वोट नहीं पढ़े जा सकते, और अनुरोध
`poll-anonymous` के साथ विफल हो जाता है। विवरण के लिए `PollVote` संरचना देखें।

[inline-code-attrs-start title = 'PollVotes Get cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** विफलता पर शामिल। **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### प्रति विकल्प वोट गिनना

परिणाम प्राप्त करने के लिए आपको इन्हें जोड़ने की आवश्यकता नहीं है - पोल अपनी गिनती रखता है। पोल को पढ़ें
`GET /api/v1/polls/:commentId` से, और इस API का उपयोग तब करें जब आपको यह जानने की आवश्यकता हो कि किसने वोट किया।

### एक पेज पर हर पोल

पूरे पेज की वोट सूची नहीं है। पूरे पेज पर रिपोर्ट करने के लिए, उसके कमेंट्स प्राप्त करें
`GET /api/v1/comments` से, जो प्रत्येक कमेंट का पोल और उसकी गिनती लौटाता है, और फिर उन पोल्स के वोट पढ़ें
जिनमें आपकी रुचि है।