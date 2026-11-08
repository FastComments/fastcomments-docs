[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

यह रूट एक [`POST /api/v1/pages/migrations`](#pages-migrations-post) जॉब से हुए हर माइग्रेशन को वापस ले लेता है। `undoId` पूरे हुए माइग्रेशन जॉब से [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) के ज़रिए प्राप्त करें। एक `undoId` का उपयोग 90 दिनों तक, और केवल एक बार किया जा सकता है।

अनडू एक बैच जॉब के रूप में चलता है, उसी तरह जैसे माइग्रेशन चलता है, और रिस्पॉन्स में उसका `batchJobId` होता है। जॉब पूरा होने पर, उसका `pageMigrationUndoResult` रिपोर्ट करता है:

- `restored`: रिकॉर्ड्स जो माइग्रेशन से पहले की उनकी वैल्यू में वापस रख दिए गए।
- `skipped`: रिकॉर्ड्स जिन्हें छोड़ दिया गया क्योंकि वे माइग्रेशन के बाद बदल गए थे, या अब मौजूद नहीं हैं। बाद में किया गया कोई एडिट कभी ओवरराइट नहीं होता।
- `notRestorable`: रिकॉर्ड्स जिन्हें माइग्रेशन ने दो पेजों को मर्ज करते समय, या डुप्लिकेट पेज रिएक्ट्स हटाते समय डिलीट कर दिया था। इन्हें वापस लाया नहीं जा सकता।

माइग्रेशन सबसे नए वाले से पहले रिवर्स किए जाते हैं, इसलिए `a` से `b` और फिर `b` से `c` जैसी श्रृंखला वापस `a` तक अनवाउंड हो जाती है। बीच में विफल हुआ माइग्रेशन जॉब भी अनडू किया जा सकता है, जो विफल होने से पहले किए गए उसके बदलावों को रिवर्स कर देता है।

अनडू को क्यू में लगाने पर कुछ खर्च नहीं होता। जॉब पूरा होने पर प्रत्येक `1000` रिस्टोर किए गए रिकॉर्ड्स पर `1` क्रेडिट लगता है, न्यूनतम `1`।

[inline-code-attrs-start title = 'Undo Page Migrations cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Undo Page Migrations रिक्वेस्ट स्ट्रक्चर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Undo Page Migrations रिस्पॉन्स स्ट्रक्चर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * विफलता पर शामिल।
     * undo-not-found (404): इस undoId वाला कोई माइग्रेशन नहीं है, या वह 90 दिनों से पुराना है।
     * already-undone (409) और undo-in-progress (409): माइग्रेशन अनडू किया जा चुका है, या उसका अनडू क्यू में है या चल रहा है।
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** विफलता पर शामिल। **/
    reason?: string
    /** प्रगति और परिणाम के लिए GET /api/v1/batch-jobs/:id में पास करें। **/
    batchJobId?: string
}
[inline-code-end]