[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

अपनी id द्वारा एकल पोल वोट पढ़ता है।

गुमनाम पोल पर दिया गया वोट नहीं पढ़ा जा सकता, और अनुरोध `poll-anonymous` के साथ विफल हो जाता है। पोल की `privacy` सेटिंग कैसे लागू होती है, यह देखने के लिए `PollVote`
संरचना देखें।

[inline-code-attrs-start title = 'PollVote Get cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Get अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Get प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found' | 'poll-not-found' | 'poll-anonymous'
    /** विफलता पर शामिल। **/
    reason?: string
    pollVote: PollVote
}
[inline-code-end]