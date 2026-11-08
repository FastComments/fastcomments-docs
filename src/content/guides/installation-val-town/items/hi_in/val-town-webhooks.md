एक val एक स्वाभाविक webhook रिसीवर है: इसका एक स्थिर URL होता है, यह सिग्नेचर सत्यापित कर सकता है, और इसमें SQLite और blob storage इनबिल्ट होते हैं।

FastComments आपके खाते के API secret से `${timestamp}.${body}` पर सिग्न करके दो हेडर भेजता है:

[inline-code-attrs-start title = 'Webhook हेडर'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix सेकंड, मिलीसेकंड नहीं
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

मेथड इवेंट को व्यक्त करता है: बनाए गए या अपडेट किए गए कमेंट के लिए **PUT**, और डिलीट किए गए कमेंट के लिए **DELETE**।

[inline-code-attrs-start title = 'डिलीवरी सत्यापित करना'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // जो सटीक बाइट्स आए हैं। c.req.json() का उपयोग करके दोबारा सीरियलाइज़ न करें।
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // पुरानी डिलीवरी अस्वीकार करें ताकि कैप्चर किया गया रिक्वेस्ट बाद में रिप्ले न हो सके।
  if (Math.abs(Math.floor(Date.now() / 1000) - Number(timestamp)) > 300) {
    return new Response("Timestamp outside window", { status: 400 });
  }

  const expected = "sha256=" + createHmac("sha256", Deno.env.get("FASTCOMMENTS_API_SECRET"))
    .update(`${timestamp}.${rawBody}`)
    .digest("hex");

  const a = new TextEncoder().encode(signature);
  const b = new TextEncoder().encode(expected);
  if (a.length !== b.length || !timingSafeEqual(a, b)) {
    return new Response("Signature mismatch", { status: 401 });
  }

  // ...JSON.parse(rawBody) को हैंडल करें
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## दो चीज़ें जो परेशान करती हैं

**रॉ बाइट्स सत्यापित करें।** JSON को पार्स करके दोबारा सीरियलाइज़ करने से key क्रम और व्हाइटस्पेस बदल जाता है, जिससे हैश अलग हो जाता है और हर डिलीवरी बिना स्पष्ट कारण के विफल हो जाती है। यही आम तौर पर वह कारण है जिससे webhook रिसीवर "बस काम नहीं करता"।

**कॉन्स्टेंट टाइम में तुलना करें।** सिग्नेचर पर साधारण `===` यह लीक कर देता है कि कितने बाइट्स मैच हुए, जो एक-एक बाइट करके फोर्ज करने के लिए पर्याप्त है।

## इवेंट हैंडल करना

जल्दी जवाब दें। FastComments नॉन-2xx पर रिट्राई करता है, और लगातार विफल होने वाला एंडपॉइंट अंततः स्वचालित रूप से डिसेबल कर दिया जाता है, इसलिए असली काम जवाब देने के बाद करें, इनलाइन नहीं।

उस काम को कमेंट id पर idempotent बनाएं। रिट्राई एक नए टाइमस्टैम्प के साथ दोबारा सिग्न किया जाता है, और एडिट और डिलीट पर वही कमेंट id फिर से आता है, इसलिए डिडुप्लिकेट करने के लिए कुछ भी स्थिर नहीं होता।