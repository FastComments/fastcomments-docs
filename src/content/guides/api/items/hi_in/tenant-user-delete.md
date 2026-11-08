[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

यह रूट `TenantUser` को id द्वारा हटाने की सुविधा प्रदान करता है।

`deleteComments` क्वेरी पैरामीटर के माध्यम से यूज़र की टिप्पणियों को हटाना संभव है। ध्यान दें कि यदि यह true है:

1. यूज़र की सभी टिप्पणियाँ लाइव हटा दी जाएँगी।
2. सभी __child__ (अब अनाथ) टिप्पणियाँ प्रत्येक टिप्पणी से संबंधित पेज कॉन्फ़िगरेशन के आधार पर हटाई या गुमनाम कर दी जाएँगी। उदाहरण के लिए यदि थ्रेड डिलीशन मोड "anonymize" है, तो रिप्लाई बने रहेंगे, और यूज़र की टिप्पणियाँ गुमनाम कर दी जाएँगी। यह केवल तब लागू होता है जब `commentDeleteMode` `Remove` हो (डिफ़ॉल्ट मान)।
3. `creditsCost` `2` हो जाता है।

### गुमनाम की गई टिप्पणियाँ

आप यूज़र की टिप्पणियों को बनाए रख सकते हैं लेकिन `commentDeleteMode=1` सेट करके उन्हें केवल गुमनाम कर सकते हैं।

यदि यूज़र की टिप्पणियाँ गुमनाम कर दी जाती हैं तो निम्नलिखित मान null सेट कर दिए जाते हैं:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` और `isDeletedUser` को `true` सेट कर दिया जाता है।

रेंडरिंग के समय, कमेंट विजेट यूज़र के नाम के लिए `DELETED_USER_PLACEHOLDER` (डिफ़ॉल्ट: "[deleted]") और टिप्पणी के लिए `DELETED_CONTENT_PLACEHOLDER` का उपयोग करेगा। इन्हें Widget Customization UI के माध्यम से अनुकूलित किया जा सकता है।

### उदाहरण

[inline-code-attrs-start title = 'TenantUser हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser हटाने का रिक्वेस्ट स्ट्रक्चर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser हटाने का रिस्पॉन्स स्ट्रक्चर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]