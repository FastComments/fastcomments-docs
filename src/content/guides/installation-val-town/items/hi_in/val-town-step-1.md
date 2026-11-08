विजेट एक स्क्रिप्ट टैग और एक कंटेनर एलिमेंट है, इसलिए यह आपके val द्वारा पहले से रेंडर की गई किसी भी चीज़ में आसानी से जुड़ जाता है। यह उदाहरण Hono JSX का उपयोग करता है, जो Val Town के HTTP टेम्पलेट्स द्वारा उपयोग किया जाने वाला है।

[inline-code-attrs-start title = 'HTTP val में कमेंट विजेट'; type='javascript' inline-code-attrs-end]
[inline-code-start]
/** @jsxImportSource npm:hono@4/jsx */
import { Hono } from "npm:hono@4";

const app = new Hono();

app.get("/:slug", (c) => {
  const slug = c.req.param("slug");
  const url = new URL(c.req.path, c.req.url).toString();

  const config = JSON.stringify({
    tenantId: "demo",
    urlId: slug,
    url,
  });

  return c.html(
    <html>
      <body>
        <h1>{slug}</h1>
        <div id="fastcomments-widget"></div>
        <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
        <script
          dangerouslySetInnerHTML={{
            __html:
              `window.FastCommentsUI(document.getElementById("fastcomments-widget"), ${config});`,
          }}
        />
      </body>
    </html>,
  );
});

export default app.fetch;
[inline-code-end]

## शिप करने से पहले एक urlId चुनें

`urlId` तय करता है कि कोई कमेंट किस थ्रेड में जाएगा। इसे अनसेट छोड़ने पर यह वर्तमान पेज URL के एक साफ़ किए गए संस्करण पर डिफ़ॉल्ट हो जाता है, जो Val Town पर बदलने वाली चीज़ है: सबडोमेन क्लेम करने तक एक val का लंबा `*.web.val.run` होस्टनाम रहता है, ब्रांचेस के अपने URL होते हैं, और पेज का नाम बदलने से पाथ बदल जाता है। हर विविधता चुपचाप एक अलग, खाली थ्रेड बन जाती है, और लक्षण ऐसा दिखता है जैसे "मेरे कमेंट्स गायब हो गए"।

इसे किसी स्थिर चीज़ पर सेट करें जिसे आप नियंत्रित करते हैं, जैसे पोस्ट स्लग या डेटाबेस id, जैसा ऊपर दिखाया गया है। `url` भी पास करें, ताकि नोटिफिकेशन ईमेल और मॉडरेशन टूल्स वास्तविक पेज से लिंक कर सकें।

## JavaScript के बिना कमेंट्स बनाए रखना

FastComments पूरे थ्रेड को सर्वर-साइड रेंडर करता है, जिसे एक val `<noscript>` ब्लॉक में डाल सकता है:

[inline-code-attrs-start title = 'बिना-JavaScript फॉलबैक'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

पैरामीटर्स को URL-encode करें। सर्वर-साइड संस्करण एनोनिमस और लॉग-इन कमेंटिंग, SSO, और नेस्टेड रिप्लाईज़ को सपोर्ट करता है।