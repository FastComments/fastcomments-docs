यह उदाहरण Wix के साथ संगतता के लिए कस्टम कोड का उपयोग करता है। **आप अन्य ट्यूटोरियल्स के FastComments कोड स्निपेट्स का उपयोग नहीं कर पाएंगे।**

हमारा कस्टम HTML डायलॉग जोड़ने के लिए फॉर्म खोलें, इसके लिए `Enter Code` पर क्लिक करें और `HTML` चुनें:

<div class="screenshot white-bg">
    <div class="title">चरण 3: HTML डायलॉग खोलें</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="चरण 3: HTML डायलॉग खोलें" />
</div>

निम्नलिखित HTML स्निपेट कॉपी करें और इसे डायलॉग में पेस्ट करें, फिर Update पर क्लिक करें:

[inline-code-attrs-start title = 'Wix कमेंट्स कोड स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

अब आपको कमेंट विजेट का एक बहुत छोटा संस्करण दिखाई देगा:

<div class="screenshot white-bg">
    <div class="title">चरण 3: परिणाम</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="चरण 3: परिणाम" />
</div>

अगले चरण में हम इसे अपने पेज के अनुरूप स्थिति और आकार देंगे।