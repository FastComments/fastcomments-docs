इसके बाद, आइए चीज़ों को इस तरह सेट करें कि कमेंट थ्रेड वर्तमान पेज के आधार पर बदले, जिससे उपयोगकर्ता वर्तमान में दिखाई जा रही सामग्री पर चर्चा कर सकें।

निम्नलिखित चरणों के बिना, आपकी पूरी साइट के लिए केवल एक ग्लोबल कमेंट थ्रेड होगा - जो बहुत उपयोगी नहीं है।

#### Dev Mode

यह कार्यक्षमता जोड़ने के लिए, हमें Wix के `Dev Mode` में जाना होगा।

स्क्रीन के ऊपर `Dev Mode` विकल्प पर क्लिक करें।

<div class="screenshot white-bg">
    <div class="title">Dev Mode सक्षम करें</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Dev Mode सक्षम करें" />
</div>

#### Element ID सेट करें

हम इसे पूरा करने के लिए कस्टम कोड जोड़ने जा रहे हैं, लेकिन पहले हमें नए embed एलिमेंट को एक ID देनी होगी ताकि हम उसे उसके नाम से संदर्भित कर सकें।

आइए इसे `fastcomments` कहते हैं।

हमारे द्वारा जोड़े गए नए embed एलिमेंट पर क्लिक करें, और dev mode में नीचे दाईं ओर आपको एक ID फ़ील्ड दिखना चाहिए जिसका मान `html1` जैसा हो:

<div class="screenshot white-bg">
    <div class="title">ID फ़ील्ड</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID फ़ील्ड" />
</div>

इसे `fastcomments` में बदलें और enter दबाएं:

<div class="screenshot white-bg">
    <div class="title">ID सेट करें</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="ID सेट करें" />
</div>

अब हम अपना कस्टम कोड जोड़ सकते हैं जो कमेंट एरिया को बताता है कि हम कौन सा पेज देख रहे हैं।

स्क्रीन के नीचे आपको इस तरह का एक कोड एडिटर दिखना चाहिए:

<div class="screenshot white-bg">
    <div class="title">एडिटर खोलें</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="ID सेट करें" />
</div>

निम्नलिखित कोड कॉपी करें और उसे वहां पेस्ट करें:

[inline-code-attrs-start title = 'Wix कमेंट्स नेविगेशन स्निपेट'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">नेविगेशन कोड जोड़ें</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="नेविगेशन कोड जोड़ें" />
</div>