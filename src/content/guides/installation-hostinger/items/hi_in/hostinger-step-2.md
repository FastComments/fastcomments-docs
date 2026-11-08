अब हम अपना विजेट कोड जोड़ते हैं।

नीचे दिया गया कोड कॉपी करें। आप यह सुनिश्चित करना चाहेंगे कि आप [fastcomments.com](https://fastcomments.com) पर साइन इन हैं 
और यदि नहीं हैं तो इस पेज को रीलोड करें, ताकि कोड आपके खाते की जानकारी के साथ पहले से भरा हो, अन्यथा यह डेमो कोड दिखाएगा।

अब कोड कॉपी करें:

[inline-code-attrs-start title = 'Hostinger Comments Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

अब अपने साइट बिल्डर पर वापस जाएं और `Enter code` पर क्लिक करें:

<div class="screenshot white-bg">
    <div class="title">Enter code</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Enter Code" />
</div>

### ध्यान दें!

यह महत्वपूर्ण है कि आप ऊपर दिया गया कोड ही उपयोग करें, अन्य दस्तावेज़ों के कोड स्निपेट नहीं, क्योंकि यह स्निपेट विशेष रूप से 
Hostinger के लिए तैयार किया गया है।

अब आपके पास निम्नलिखित जैसा कुछ होना चाहिए, जो खाली दिखाई देता है। यह अपेक्षित है। अपने माउस को उस क्षेत्र पर ले जाएं 
जहां विजेट होना चाहिए:

<div class="screenshot white-bg">
    <div class="title">Code Widget Added</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Code Widget Added" />
</div>

अब विजेट को वांछित आकार में खींचें, आप इसे दिखाई देता देखेंगे:

<div class="screenshot white-bg">
    <div class="title">Resize It</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Resize It" />
</div>

...और अब प्रीव्यू करें और सेव करें!