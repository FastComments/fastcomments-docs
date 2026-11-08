[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

यह API उपयोगकर्ता को दिखाने के लिए कमेंट्स प्राप्त करने के लिए उपयोग की जाती है। उदाहरण के लिए, यह स्वचालित रूप से अस्वीकृत या स्पैम कमेंट्स को फ़िल्टर कर देती है।

### पेजिनेशन

पेजिनेशन दो तरीकों में से किसी एक से किया जा सकता है, जो परफ़ॉर्मेंस आवश्यकताओं और उपयोग के मामले पर निर्भर करता है:

1. सबसे तेज़: **Precalculated Pagination**:
   1. जब आप हमारे प्रीबिल्ट विजेट्स और क्लाइंट्स का उपयोग करते हैं, तो FastComments इसी तरह काम करता है।
   2. "next" पर क्लिक करना केवल पेज काउंट बढ़ाता है।
   3. आप इसे key-value स्टोर से प्राप्त किए गए डेटा की तरह सोच सकते हैं।
   4. इस तरह, बस `0` से शुरू होने वाला एक `page` पैरामीटर और `direction` के रूप में सॉर्ट दिशा परिभाषित करें।
   5. पेज साइज़ कस्टमाइज़ेशन नियमों के माध्यम से अनुकूलित किया जा सकता है।
2. सबसे लचीला: **Flexible Pagination**:
   1. इस तरह आप कस्टम `limit` और `skip` पैरामीटर परिभाषित कर सकते हैं। `page` पास न करें।
   2. सॉर्ट `direction` भी समर्थित है।
   3. `limit`, `skip` लागू होने के बाद लौटाने की कुल संख्या है।
      - उदाहरण: `page size = 100` और `page = 2` होने पर `skip = 200, limit = 100` सेट करें।
   4. चाइल्ड कमेंट्स भी पेजिनेशन में गिने जाते हैं। आप `asTree` विकल्प का उपयोग करके इससे बच सकते हैं।
      - आप `limitChildren` और `skipChildren` के माध्यम से चाइल्ड कमेंट्स को पेजिनेट कर सकते हैं।
      - आप `maxTreeDepth` के माध्यम से लौटाए गए थ्रेड्स की गहराई सीमित कर सकते हैं।

### थ्रेड्स

1. `Precalculated Pagination` का उपयोग करते समय, कमेंट्स *page* के अनुसार समूहित होते हैं और थ्रेड्स में कमेंट्स समग्र पेज को प्रभावित करते हैं।
   1. इस तरह, थ्रेड्स को क्लाइंट पर `parentId` के आधार पर निर्धारित किया जा सकता है।
   2. उदाहरण के लिए, एक टॉप-लेवल कमेंट और 29 रिप्लाई वाले पेज के लिए, और API में `page=0` सेट करने पर - आपको केवल टॉप-लेवल कमेंट और 29 चाइल्ड मिलेंगे।
2. `Flexible Pagination` का उपयोग करते समय, आप एक `parentId` पैरामीटर परिभाषित कर सकते हैं।
   1. केवल टॉप-लेवल कमेंट्स प्राप्त करने के लिए इसे null सेट करें।
   2. फिर थ्रेड्स देखने के लिए, API को फिर से कॉल करें और `parentId` पास करें।
   3. एक सामान्य समाधान यह है कि टॉप-लेवल कमेंट्स के लिए एक API कॉल करें और फिर प्रत्येक कमेंट के चाइल्ड के लिए समानांतर API कॉल करें।
3. __नया - फ़रवरी 2023 से!__ `&asTree=true` का उपयोग करके ट्री के रूप में प्राप्त करें।
   1. आप इसे `Flexible Pagination as a Tree` की तरह सोच सकते हैं।
   2. पेजिनेशन में केवल टॉप-लेवल कमेंट्स गिने जाते हैं।
   3. ट्री को रूट से शुरू करने के लिए `parentId=null` सेट करें (आपको `parentId` सेट करना अनिवार्य है)।
   4. पेजिनेशन के लिए `skip` और `limit` सेट करें।
   5. `asTree` को `true` सेट करें।
   6. क्रेडिट्स लागत `2x` बढ़ जाती है, क्योंकि इस परिदृश्य में हमारे बैकएंड को बहुत अधिक काम करना पड़ता है।
   7. `maxTreeDepth`, `limitChildren`, और `skipChildren` को आवश्यकतानुसार सेट करें।

### ट्रीज़ की व्याख्या

`asTree` का उपयोग करते समय, पेजिनेशन को समझना कठिन हो सकता है। यहाँ एक उपयोगी ग्राफ़िक है:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### उपयोगकर्ता के संदर्भ में कमेंट्स प्राप्त करना

`/comments` API को दो संदर्भों में, विभिन्न उपयोग के मामलों के लिए उपयोग किया जा सकता है:

- अपना स्वयं का क्लाइंट बनाने के लिए जानकारी के साथ सॉर्ट और टैग किए गए कमेंट्स लौटाने के लिए।
  - इस मामले में, एक `contextUserId` क्वेरी पैरामीटर परिभाषित करें।
- कस्टम इंटीग्रेशन के लिए अपने बैकएंड से कमेंट्स प्राप्त करने के लिए।
  - `contextUserId` के बिना प्लेटफ़ॉर्म डिफ़ॉल्ट रूप से इसी का उपयोग करेगा।

[inline-code-attrs-start title = 'कमेंट्स Precalculated Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'कमेंट्स Flexible Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'उपयोगकर्ता संदर्भ में कमेंट्स Flexible Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'केवल टॉप-लेवल कमेंट्स के लिए उपयोगकर्ता संदर्भ में कमेंट्स Flexible Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### कमेंट्स को ट्री के रूप में प्राप्त करें

कमेंट्स को ट्री के रूप में प्राप्त करना संभव है, जिसमें पेजिनेशन केवल टॉप-लेवल कमेंट्स को गिनता है।

[inline-code-attrs-start title = 'उपयोगकर्ता संदर्भ में ट्री के रूप में कमेंट्स'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

केवल टॉप-लेवल कमेंट्स और तुरंत चाइल्ड प्राप्त करना चाहते हैं? यहाँ एक तरीका है:

[inline-code-attrs-start title = 'अधिकतम गहराई के साथ ट्री के रूप में कमेंट्स'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

हालाँकि, अपने UI में आपको यह जानने की आवश्यकता हो सकती है कि प्रत्येक कमेंट पर "show replies" बटन दिखाना है या नहीं। ट्री के माध्यम से कमेंट्स प्राप्त करते समय, जब लागू हो, तो कमेंट्स पर एक `hasChildren` प्रॉपर्टी टैग की जाती है।

### ट्री के रूप में कमेंट्स प्राप्त करें, हैश टैग द्वारा खोज

API का उपयोग करके हैशटैग द्वारा खोज करना संभव है, आपके पूरे टेनेंट में (एक पेज या `urlId` तक सीमित नहीं)।

इस उदाहरण में, हम `urlId` को छोड़ देते हैं, और कई हैशटैग द्वारा खोज करते हैं। API केवल वही कमेंट्स लौटाएगा जिनमें सभी अनुरोधित हैशटैग मौजूद हैं।

[inline-code-attrs-start title = 'उपयोगकर्ता संदर्भ में हैश टैग द्वारा ट्री के रूप में कमेंट्स'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### सभी रिक्वेस्ट पैरामीटर्स

[inline-code-attrs-start title = 'कमेंट्स रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### रिस्पॉन्स

[inline-code-attrs-start title = 'कमेंट्स रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### उपयोगी टिप्स

#### URL ID

संभवतः आप `Comment` API का उपयोग `urlId` पैरामीटर के साथ करना चाहेंगे। आप पहले `Pages` API को कॉल कर सकते हैं, यह देखने के लिए कि आपके लिए उपलब्ध `urlId` मान कैसे दिखते हैं।

#### अनाम (Anonymous) क्रियाएँ

अनाम कमेंटिंग के लिए संभवतः आप कमेंट्स प्राप्त करते समय, और फ्लैगिंग तथा ब्लॉकिंग करते समय `anonUserId` पास करना चाहेंगे।

(!) यह कई ऐप स्टोर्स के लिए आवश्यक है क्योंकि उपयोगकर्ताओं को उपयोगकर्ता-निर्मित कंटेंट को फ्लैग करने में सक्षम होना चाहिए जिसे वे देख सकते हैं, भले ही वे लॉग इन न हों। ऐसा न करने पर आपका ऐप उस स्टोर से हटाया जा सकता है।

#### कमेंट्स लौटाए नहीं जा रहे हैं

जाँचें कि आपके कमेंट्स अप्रूव किए गए हैं, और स्पैम नहीं हैं।