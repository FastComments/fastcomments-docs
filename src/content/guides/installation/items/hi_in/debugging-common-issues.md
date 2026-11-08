यहाँ कुछ लक्षण दिए गए हैं जिनका सामना हम अक्सर करते हैं, और उनके सामान्य समाधान।

### "This is a demo" संदेश

यह तब दिखाया जाता है जब आपने हमारे होम पेज से विजेट कोड कॉपी किया है, जो हमारे डेमो
टेनेंट का उपयोग करता है। अपने टेनेंट का उपयोग करने के लिए, विजेट कोड [यहाँ से](https://fastcomments.com/auth/my-account/get-acct-code) कॉपी करें।

### "FastComments cannot load on this domain" त्रुटि

FastComments को यह जानने की आवश्यकता है कि कौन से डोमेन आपके स्वामित्व में हैं, ताकि आपके खाते से जुड़े
अनुरोधों को प्रमाणित किया जा सके। इस त्रुटि को हल करने का तरीका देखने के लिए [हमारा दस्तावेज़ देखें](/guide-multiple-sites.html#add-domains-to-account) (बस सटीक सबडोमेन + डोमेन अपने खाते में जोड़ें)।

ध्यान दें कि यह केवल ट्रायल अवधि समाप्त होने के बाद ही होना चाहिए। ट्रायल अवधि के दौरान, नए डोमेन से आने वाले किसी भी अनुरोध
को स्वचालित रूप से आपके खाते में जोड़ दिया जाएगा।

### कस्टम इंस्टॉलेशन के लिए माइग्रेट की गई टिप्पणियाँ नहीं दिख रही हैं

आमतौर पर यह तब होता है जब आयात की गई टिप्पणियाँ किसी `Page ID` से जुड़ी होती हैं, और आप एक URL पास कर रहे हैं
(या कोई मान नहीं, जिस स्थिति में यह पेज URL पर डिफ़ॉल्ट होता है)।

आप इसे [अपनी टिप्पणियाँ निर्यात करके](https://fastcomments.com/auth/my-account/manage-data/export) और `URL ID` कॉलम देखकर डिबग कर सकते हैं (वर्तमान में कॉलम `B`)।

सुनिश्चित करें कि `URL ID` कॉलम में दिखाई देने वाले मान वही मान हैं जो आप विजेट
कॉन्फ़िगरेशन को `urlId` पैरामीटर के रूप में पास कर रहे हैं।

अधिक विवरण के लिए, हमारा [How Comments are Tied to Pages and Articles दस्तावेज़](/guide-customizations-and-configuration.html#url-id) पढ़ने का प्रयास करें।

यदि सब विफल हो जाए, तो [हमसे संपर्क करें](https://fastcomments.com/auth/my-account/help)।

### कमेंट विजेट नहीं दिख रहा है

यदि कमेंट विजेट नहीं दिख रहा है, तो त्रुटियों के लिए Chrome डेवलपर कंसोल की जाँच करें।

अधिकांश गलत कॉन्फ़िगरेशन के लिए, कमेंट विजेट लोड होने में सक्षम होने पर कम से कम पेज पर एक त्रुटि दिखाएगा। कुछ भी न दिखना आमतौर पर स्क्रिप्टिंग त्रुटि का संकेत होता है।

### वांछित कॉन्फ़िगरेशन अपेक्षा के अनुसार काम नहीं कर रहा है

यह देखने के लिए कि कमेंट विजेट को क्या कॉन्फ़िगरेशन पास किया जा रहा है, हमारे [Chrome एक्सटेंशन](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) का उपयोग करें। यदि सब विफल हो जाए, तो Chrome एक्सटेंशन द्वारा दिखाए गए का स्क्रीनशॉट लें
और [हमसे संपर्क करें](https://fastcomments.com/auth/my-account/help)।

### उसी URL पर अलग हैश बैंग के साथ टिप्पणियाँ गायब

डिफ़ॉल्ट रूप से, FastComments उस "बकेट" के लिए पेज URL का उपयोग करेगा जहाँ टिप्पणियाँ संग्रहीत होती हैं। यदि आपके URLs में `#hashbangs` शामिल हैं, और ये `#hashbangs`
टिप्पणी थ्रेड की पहचान करने वाले पहचानकर्ता का हिस्सा नहीं होने चाहिए, तो हम हैश बैंग मान को सरलता से अनदेखा कर सकते हैं, उदाहरण के लिए:

[inline-code-attrs-start title = 'हैश बैंग अनदेखा करने का उदाहरण'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

ध्यान दें कि इस परिवर्तन के बाद, मौजूदा टिप्पणियों के लिए एक माइग्रेशन करना होगा। [इसके लिए, हमसे संपर्क करें।](https://fastcomments.com/auth/my-account/help)

### URL क्वेरी पैरामीटर विजेट को प्रभावित कर रहे हैं

डिफ़ॉल्ट रूप से, FastComments उस "बकेट" के लिए पेज URL का उपयोग करेगा जहाँ टिप्पणियाँ संग्रहीत होती हैं। यदि आपके URLs में ऐसे क्वेरी पैरामीटर शामिल हैं
जो टिप्पणी थ्रेड की पहचान करने वाले पहचानकर्ता का हिस्सा नहीं होने चाहिए, तो हम उन्हें सरलता से अनदेखा कर सकते हैं, उदाहरण के लिए:

[inline-code-attrs-start title = 'क्वेरी पैरामीटर अनदेखा करें'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

ध्यान दें कि इस परिवर्तन के बाद, मौजूदा टिप्पणियों के लिए एक माइग्रेशन करना होगा। [इसके लिए, हमसे संपर्क करें।](https://fastcomments.com/auth/my-account/help)

### ईमेल प्राप्त नहीं हो रहे हैं

FastComments पर, हम यह सुनिश्चित करने में बहुत मेहनत करते हैं कि हमारी ईमेल डिलीवरी यथासंभव
विश्वसनीय हो। हालाँकि, कुछ ईमेल प्रदाताओं को विश्वसनीय रूप से डिलीवरी करना कठिन है। fastcomments.com से आने वाले संदेशों के लिए अपना स्पैम
फ़ोल्डर जाँचें।

यदि आप [हमसे संपर्क करते हैं](https://fastcomments.com/auth/my-account/help), तो हम आमतौर पर इस बारे में अधिक जानकारी दे सकते हैं कि आपको हमसे ईमेल क्यों नहीं दिख रहे हैं।