इसका नाम Domain Migration टूल होने के बावजूद, इसका उपयोग अलग-अलग पेजों के बीच कमेंट्स माइग्रेट करने के लिए भी किया जा सकता है।

बस पुरानी और नई [urlId](/guide-customizations-and-configuration.html#url-id) वैल्यू `from` और `to` इनपुट फ़ील्ड में दर्ज करें।

### API के माध्यम से पेज माइग्रेट करना

एक साथ कई पेज माइग्रेट करने के लिए, [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post) का उपयोग करें। यह प्रति रिक्वेस्ट 100 `from` और `to` पेयर तक लेता है और उन्हें बैच जॉब के रूप में चलाता है।

`type` को `urlId` पर सेट करने पर, केवल वही पेज मूव होता है जिसका `urlId` बिल्कुल `from` के बराबर होता है, इसलिए `post-1`, `post-10` से मैच नहीं करता। डैशबोर्ड टूल में यह विकल्प नहीं है।

प्रत्येक API माइग्रेशन को 90 दिनों तक [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post) के साथ पूर्ववत किया जा सकता है।