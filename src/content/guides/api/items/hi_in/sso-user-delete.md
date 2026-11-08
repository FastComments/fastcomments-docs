[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

यह रूट एक SSO यूज़र को उनकी id के आधार पर हटाने की सुविधा प्रदान करता है।

ध्यान दें कि इस यूज़र के लिए पेलोड के साथ कमेंट विजेट को दोबारा लोड करने पर यूज़र बिना किसी बाधा के आसानी से फिर से बन जाएगा।

`deleteComments` क्वेरी पैरामीटर के माध्यम से यूज़र के कमेंट्स को हटाना संभव है। ध्यान दें कि यदि यह true है:

1. यूज़र के सभी कमेंट्स लाइव हटा दिए जाएंगे।
2. सभी __child__ (अब अनाथ) कमेंट्स प्रत्येक कमेंट से जुड़े पेज कॉन्फ़िगरेशन के आधार पर हटा दिए जाएंगे या गुमनाम (anonymize) कर दिए जाएंगे। उदाहरण के लिए यदि थ्रेड डिलीशन मोड "anonymize" है, तो रिप्लाई बने रहेंगे, और यूज़र के कमेंट्स गुमनाम कर दिए जाएंगे। यह केवल तब लागू होता है जब `commentDeleteMode` `Remove` हो (डिफ़ॉल्ट मान)।
3. `creditsCost` `2` हो जाता है।

### गुमनाम किए गए कमेंट्स

आप यूज़र के कमेंट्स को बनाए रख सकते हैं लेकिन `commentDeleteMode=1` सेट करके उन्हें केवल गुमनाम कर सकते हैं।

यदि यूज़र के कमेंट्स गुमनाम कर दिए गए हैं तो निम्नलिखित मान null सेट कर दिए जाते हैं:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` और `isDeletedUser` को `true` सेट कर दिया जाता है।

रेंडरिंग के समय, कमेंट विजेट यूज़र के नाम के लिए `DELETED_USER_PLACEHOLDER` (डिफ़ॉल्ट: "[deleted]") और कमेंट के लिए `DELETED_CONTENT_PLACEHOLDER` का उपयोग करेगा। इन्हें Widget Customization UI के माध्यम से कस्टमाइज़ किया जा सकता है।

### उदाहरण

[inline-code-attrs-start title = 'SSOUser हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser हटाने की रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // डिफ़ॉल्ट
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** यूज़र के कमेंट्स को भी हटाने के लिए आप इसे true सेट कर सकते हैं। इससे क्रेडिट लागत दोगुनी हो जाएगी। **/
    deleteComments?: 'true' | 'false'
    /** यूज़र के कमेंट्स को कैसे हैंडल करना है, यह तय करने के लिए आप इसे अपनी इच्छानुसार सेट कर सकते हैं। **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser हटाने की रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** विफलता पर शामिल। **/
    reason?: string
    user?: SSOUser; // सफलता पर हम हटाए गए यूज़र को वापस करते हैं।
}
[inline-code-end]