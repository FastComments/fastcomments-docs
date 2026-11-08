एक `Vote` ऑब्जेक्ट किसी उपयोगकर्ता द्वारा दिए गए वोट को दर्शाता है।

कमेंट्स और वोट के बीच संबंध `commentId` के माध्यम से परिभाषित होता है।

Vote ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'Vote संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Vote {
    id: string
    urlId: string
    commentId: string
    userId: string
    direction: 1 | -1
    createdAt: string
}
[inline-code-end]