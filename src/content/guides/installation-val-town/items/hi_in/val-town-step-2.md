`tenantId: "demo"` एक साझा सार्वजनिक सैंडबॉक्स है। यह बिना साइनअप के काम करता है, इसीलिए उदाहरण इसका उपयोग करते हैं, लेकिन FastComments आज़माने वाले सभी अन्य लोग उन्हीं थ्रेड्स में लिखते हैं और कोई भी उन्हें मॉडरेट कर सकता है। जिस चीज़ की आपको परवाह है, उसे प्रकाशित करने से पहले इसे बदल लें।

आपकी tenant ID [API secret पेज](https://fastcomments.com/auth/my-account/api-secret) पर मौजूद है।

Tenant ID सार्वजनिक होती है और ब्राउज़र कोड में रहनी चाहिए। API secret ऐसा नहीं है, और इस पेज पर किसी चीज़ को इसकी आवश्यकता नहीं है।

## इसे एनवायरनमेंट वेरिएबल से पढ़ें

Val Town के वैल्स फ्री टियर पर सार्वजनिक होते हैं, इसलिए उनका सोर्स सबको दिखाई देता है। किसी भी संवेदनशील चीज़ को एनवायरनमेंट वेरिएबल में रखें और `Deno.env.get` से पढ़ें:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// केवल eu.fastcomments.com पर बनाए गए अकाउंट्स इसे "eu" पर सेट करते हैं।
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

यह Val Town पर एक दूसरे कारण से भी आमतौर पर ज़रूरी से ज़्यादा मायने रखता है: **किसी वैल को रीमिक्स करने पर एनवायरनमेंट वेरिएबल की कुंजियाँ कॉपी होती हैं, लेकिन उनके मान नहीं।** एनवायरनमेंट वेरिएबल में रखा गया secret आपके वैल के साथ किसी और के अकाउंट में नहीं जाता। फ़ाइल में लिखा गया secret जाता है।

`"demo"` पर फ़ॉलबैक वैल को उन सभी के लिए काम करते रखता है जो अपनी tenant सेट करने से पहले इसे रीमिक्स करते हैं।

## EU अकाउंट्स

एक अकाउंट, उसका डेटा और उसकी कुंजियाँ एक ही क्षेत्र में रहते हैं। यदि आपका अकाउंट `eu.fastcomments.com` पर बनाया गया था, तो हर विजेट कॉन्फ़िग में `region: "eu"` भी चाहिए, और स्क्रिप्ट्स `cdn-eu.fastcomments.com` से लोड होती हैं। अन्यथा दोनों को वैसे ही छोड़ दें।