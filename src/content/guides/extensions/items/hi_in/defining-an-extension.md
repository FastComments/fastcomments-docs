सबसे छोटा संभव एक्सटेंशन इस प्रकार होगा:

[inline-code-attrs-start title = 'एक सरल एक्सटेंशन'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

इस उदाहरण के लिए, इसे `my-extension.js` के रूप में सेव करें, और इसे `https://example.com/my-extension.min.js` पर उपलब्ध कराएं।

यह एक्सटेंशन कुछ नहीं करता, सिवाय इसके कि लोड होने पर यह कोर कमेंट लाइब्रेरी द्वारा बनाए गए एक्सटेंशन ऑब्जेक्ट को फ़ेच करता है।

यह `Extension` ऑब्जेक्ट एक सिंगलटन है और किसी अन्य एक्सटेंशन के साथ साझा नहीं किया जाता।

अगला, अपना एक्सटेंशन लोड करने के लिए, हमें कमेंट विजेट को इसके बारे में बताना होगा। उदाहरण के लिए:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'कस्टम एक्सटेंशन का उपयोग करना'; code-example-end]

कार्यात्मक उदाहरणों के लिए, अगला अनुभाग देखें।