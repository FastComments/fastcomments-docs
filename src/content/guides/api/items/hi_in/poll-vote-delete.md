[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

एक वोट वापस लेता है। जिस विकल्प पर वोट दिया गया था वह अपनी गिनती वापस प्राप्त कर लेता है, और मतदाता दोबारा वोट देने के लिए स्वतंत्र होता है।

[inline-code-attrs-start title = 'PollVote Delete cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Delete अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Delete प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    /** अपडेट की गई गिनती के साथ पोल। **/
    poll: CommentPoll
}
[inline-code-end]

### अन्य नोट्स

- एक ही वोट को दो बार डिलीट करने पर दूसरी बार `not-found` प्रतिक्रिया मिलती है, और गिनती में कोई बदलाव नहीं होता।
- यदि वोट देने के बाद पोल बदल दिया गया है, तो वोट हटा दिया जाता है लेकिन गिनती में कोई बदलाव नहीं होता, क्योंकि
  बदले गए पोल की गिनती शून्य से शुरू होती है।