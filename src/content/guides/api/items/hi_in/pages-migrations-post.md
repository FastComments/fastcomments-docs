[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

यह रूट कमेंट्स और पेजों को एक `urlId` या डोमेन से दूसरे में स्थानांतरित करता है, ठीक वैसे ही जैसे डैशबोर्ड में [Domain Migration tool](/guide-migrations.html#migrating-domains) काम करता है। प्रति रिक्वेस्ट अधिकतम 100 माइग्रेशन भेजे जा सकते हैं। ये दिए गए क्रम में चलते हैं, इसलिए `a` से `b` के बाद `b` से `c` करना काम करता है।

प्रत्येक माइग्रेशन का एक `type` होता है:

- `urlId` उस पेज को स्थानांतरित करता है जिसका `urlId` बिल्कुल `from` के बराबर हो। `post-1`, `post-10` से मेल नहीं खाता।
- `domain` हर `urlId` और `url` के अंदर `from` को खोजकर बदलता है, और प्रत्येक कमेंट का `domain` अपडेट करता है। दोनों मान एक डोमेन होने चाहिए, जैसे `old.example.com`।

कमेंट्स, पेज, पेज रिएक्ट्स, वोट्स, नोटिफिकेशन और सब्सक्रिप्शन — सब स्थानांतरित हो जाते हैं। यदि `to` पर पहले से कोई पेज मौजूद है, तो दोनों पेज मर्ज हो जाते हैं।

माइग्रेशन एक बैच जॉब के रूप में चलता है। रिस्पॉन्स में एक `batchJobId` होता है। इसकी प्रगति देखने के लिए [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) को पोल करें। जॉब पूरा होने पर प्रत्येक माइग्रेशन के लिए एक रिज़ल्ट और एक `undoId` होता है, जिसे आप 90 दिनों के भीतर [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) को पास कर सकते हैं।

रिक्वेस्ट कतार में जोड़े जाने से पहले वैलिडेट की जाती है। यदि कोई भी माइग्रेशन अमान्य है, तो पूरी रिक्वेस्ट `422` के साथ विफल हो जाती है और कुछ भी नहीं चलता।

माइग्रेशन कतार में जोड़ने पर कुछ खर्च नहीं होता। जॉब पूरा होने पर प्रति `1000` माइग्रेट किए गए कमेंट्स पर `1` क्रेडिट खर्च होता है, न्यूनतम `1`।

[inline-code-attrs-start title = 'Page Migrations cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page Migrations रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** दिए गए क्रम में चलते हैं। अधिकतम 100। **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** अधिकतम 1000 वर्ण। शुरुआती और अंतिम व्हाइटस्पेस हटा दिया जाता है। **/
    from: string
    /** "from" से अलग होना चाहिए। अधिकतम 1000 वर्ण। **/
    to: string
    /** "urlId" केवल सटीक urlId से मेल खाता है। "domain" हर urlId और url के अंदर मान को खोजकर बदलता है। **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Migrations रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** विफलता पर शामिल। invalid-migration के लिए, यह पहले अमान्य माइग्रेशन का इंडेक्स बताता है। **/
    reason?: string
    /** प्रगति और रिज़ल्ट के लिए GET /api/v1/batch-jobs/:id को पास करें। **/
    batchJobId?: string
}
[inline-code-end]

#### नोट्स

- एक ही टेनेंट के जॉब्स एक समय में एक ही चलते हैं, सबसे पुराना पहले। दूसरा माइग्रेशन पहले के पूरा होने की प्रतीक्षा करता है।
- नए `urlId` के साथ `PATCH /api/v1/pages/:id` केवल पेज को बदलता है, उसके कमेंट्स को नहीं। किसी पेज के कमेंट्स को स्थानांतरित करने के लिए इस रूट का उपयोग करें।