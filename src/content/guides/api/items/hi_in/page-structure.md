एक `Page` ऑब्जेक्ट उस पेज को दर्शाता है जिससे कई कमेंट्स संबंधित हो सकते हैं। यह संबंध
`urlId` द्वारा परिभाषित होता है।

एक `Page` पेज का शीर्षक, कमेंट काउंट और `urlId` जैसी जानकारी संग्रहीत करता है।

Page ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'Page संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** इसे null पर सेट करने का अर्थ है कि सभी SSO उपयोगकर्ता पेज देख सकते हैं। खाली सूची का अर्थ है कि यह सभी उपयोगकर्ताओं के लिए बंद है। **/
    accessibleByGroupIds?: string[] | null
    /** क्या यह पेज नए कमेंट्स के लिए बंद है? **/
    isClosed?: boolean
}
[inline-code-end]