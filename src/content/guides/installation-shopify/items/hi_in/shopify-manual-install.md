यदि आप [Shopify App Store ऐप](https://apps.shopify.com/fastcomments) इंस्टॉल नहीं कर सकते, तो भी आप अपनी थीम को एडिट करके FastComments जोड़ सकते हैं। यह तरीका तब उपयोगी होता है जब आप किसी ऐसे FastComments टेनेंट को कनेक्ट करना चाहते हैं जिसके आप पहले से मालिक हैं, या जब आप किसी ऐसे Shopify स्टोरफ्रंट पर एम्बेड कर रहे हों जहाँ ऐप एक विकल्प नहीं है।

अधिकांश स्टोर के लिए ऐप-आधारित इंस्टॉल ही समर्थित तरीका है। इसका उपयोग तभी करें जब ऐप उपयुक्त न हो।

### चरण 1: Shopify के नेटिव कमेंट्स अक्षम करें

अपने Shopify एडमिन में, **Blog posts > Manage blogs** पर जाएं, प्रत्येक ब्लॉग खोलें, और दाईं ओर के पैनल में **Comments are disabled** सेट करें। सेव करें।

इससे Shopify के बिल्ट-इन कमेंटिंग का FastComments के साथ दिखना बंद हो जाएगा।

### चरण 2: ब्लॉग थीम टेम्पलेट खोलें

अपने Shopify एडमिन में:

1. **Online Store > Themes** पर जाएं।
2. अपनी वर्तमान थीम के नीचे, **Actions > Edit code** पर क्लिक करें।
3. बाईं ओर के फ़ाइल ब्राउज़र में, **Sections** खोलें और `main-article.liquid` पर क्लिक करें।

यह वह टेम्पलेट है जिसे Shopify किसी एकल ब्लॉग आर्टिकल के लिए रेंडर करता है।

### चरण 3: FastComments स्निपेट पेस्ट करें

`main-article.liquid` की लगभग लाइन 100 तक स्क्रॉल करें, आर्टिकल बॉडी के समापन `</div>` के ठीक बाद। निम्नलिखित स्निपेट पेस्ट करें:

[inline-code-attrs-start title = 'Shopify FastComments स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

`"demo"` को [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret) से अपने स्वयं के Tenant ID से बदलें। **Save** पर क्लिक करें।

### चरण 4: अपने शॉप डोमेन को अधिकृत करें

अपने लाइव स्टोर पर एक ब्लॉग पोस्ट खोलें। यदि आप कमेंट विजेट के बजाय एक अधिकृति त्रुटि देखते हैं, तो FastComments को यह जानने की आवश्यकता है कि आपका स्टोर इस टेनेंट का उपयोग करने के लिए अनुमति प्राप्त है। [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors) देखें।

### अन्य पेजों पर FastComments जोड़ना

वही स्निपेट किसी भी Liquid टेम्पलेट पर काम करता है, जिसमें प्रोडक्ट पेज, कस्टम पेज और होम पेज शामिल हैं। इसे उस जगह पेस्ट करें जहाँ आप कमेंट्स दिखाना चाहते हैं और यदि आप प्रति पेज एक स्थिर पहचानकर्ता चाहते हैं तो `urlId` को समायोजित करें (उदाहरण के लिए, प्रोडक्ट टेम्पलेट पर `urlId: "{{ product.id }}"`)।