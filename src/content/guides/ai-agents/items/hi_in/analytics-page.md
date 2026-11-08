Analytics क्रॉस-एजेंट डैशबोर्ड है। यह AI Agents पेज से **Analytics** टैब (पूरे tenant के लिए) या प्रत्येक एजेंट की row पर मौजूद **Analytics** बटन से per-agent एक्सेस किया जा सकता है।

### Filter

ऊपर एक ड्रॉपडाउन - **All agents** या कोई विशिष्ट एजेंट। पेज का बाकी हिस्सा उसी के अनुसार रीस्कोप हो जाता है।

### Budget usage

चार प्रोग्रेस बार, जो cap के मुकाबले वर्तमान अवधि का खर्च दिखाते हैं:

- **Agent today** (जब किसी विशिष्ट एजेंट पर फ़िल्टर किया गया हो) - दैनिक एजेंट cap।
- **Agent this month** - मासिक एजेंट cap।
- **Account today** - tenant की दैनिक cap।
- **Account this month** - tenant की मासिक cap।

जब कोई cap सेट नहीं होती, तो बार "(no cap set)" दिखाता है और कुल खर्च दिखाता है।

### Daily cost (last 30 days)

चयनित scope के लिए आपके tenant की मुद्रा में प्रति-दिन लागत की तालिका। यह निम्न को पहचानने के लिए उपयोगी है:

- **Sudden cost spikes** - आमतौर पर किसी runaway loop या किसी viral comment के triggers को फैलाने के कारण।
- **Cost drift** - आपके समुदाय के बढ़ने के साथ धीरे-धीरे बढ़ती दैनिक लागत।

### Actions taken

वर्तमान महीने में action प्रकारों का विवरण - "Wrote a comment: 47", "Marked a comment as spam: 12", आदि। यह जाँचने के लिए उपयोगी है कि एजेंट वही कर रहा है जिसकी आप अपेक्षा करते थे।

### Triggers skipped (this month)

[drop reason](#drop-reasons) के अनुसार समूहीकृत गिनती:

- Over agent daily / agent monthly / account daily / account monthly।
- Rate-limited।
- Concurrency saturated।

यदि आपको यहाँ drops दिखते हैं, तो आपका एजेंट budget या rate limit से टकरा रहा है और उन triggers को छोड़ रहा है जिन पर अन्यथा चलता। [Drop Reasons](#drop-reasons) देखें।

### Dry-run vs live (this month)

- **Enabled runs** - इस महीने वास्तविक actions लेने वाले runs की गिनती।
- **Dry runs** - इस महीने dry-run मोड में runs की गिनती।

एक उपयोगी ट्यूनिंग संकेत: एक बिल्कुल नया एजेंट जो अभी तक Enabled में promote नहीं हुआ है, वह केवल dry runs दिखाएगा। Enabled स्थिति में एक एजेंट जिसकी इस सेक्शन में सभी गिनतियाँ शून्य हैं, वह निष्क्रिय बैठा है - या तो वह trigger नहीं हो रहा, या उसे scope से बाहर रखा गया है, या उसके triggers सही ढंग से कॉन्फ़िगर नहीं हैं।

### Top agents by monthly cost

जब फ़िल्टर **All agents** हो, तो पेज महीने-अब-तक की लागत के अनुसार एजेंटों की सूची दिखाता है। अपने सबसे महँगे एजेंट को पहचानना लागत अनुकूलन का पहला कदम है - आमतौर पर जवाब होता है "उसके [context options](#context-options) कड़े करें" या "उसकी [budget cap](#budgets-overview) कम करें"।

### Agents at or near their cap

उन एजेंटों का per-agent विवरण जिनका खर्च वर्तमान अवधि में उनकी per-agent caps पर या उसके निकट है:

- **near cap** - cap के एक कॉन्फ़िगर करने योग्य प्रतिशत से अधिक।
- **over cap** - वास्तव में capped, उस अवधि में `{count} dropped` triggers के साथ।

इस तालिका से एजेंट पर क्लिक करें ताकि cap बढ़ाई जा सके, scope सीमित किया जा सके, या उसे pause किया जा सके।

### Account summary

जब फ़िल्टर **All agents** हो:

- **Triggers today** - गिनती।
- **Triggers this month** - गिनती।
- प्रत्येक के लिए: एक `dropped` suffix जो दिखाता है कि कितने छोड़े गए।

### Currency

सभी मौद्रिक मान आपके tenant की मुद्रा में दिखाए जाते हैं।

### What this page does not do

- यह **per-action cost breakdowns** नहीं दिखाता - वे [Run Detail View](#run-detail-view) पर होते हैं।
- यह **transcripts** या **LLM responses** नहीं दिखाता।
- यह आपको एजेंटों पर action लेने की अनुमति नहीं देता - editing, pausing, deleting सभी एजेंट सूची / edit पेज से किए जाते हैं।