[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments को अनुकूलित (customize) करने के लिए डिज़ाइन किया गया है, और हमारे विजेट्स द्वारा उपयोग किया जाने वाला फ़ॉन्ट भी इसका अपवाद नहीं है।

डिफ़ॉल्ट रूप से, FastComments विभिन्न प्रकार के डिवाइसेस पर सर्वोत्तम दिखने के लिए `system font stack` का उपयोग करता है।

अपने स्वयं के फ़ॉन्ट परिभाषित करने के लिए, [Custom CSS दस्तावेज़ीकरण](/guide-customizations-and-configuration.html#custom-css) देखें।

वहाँ आपको कस्टम CSS परिभाषित करने का तरीका मिलेगा, जो आपको अपने इच्छित फ़ॉन्ट परिभाषित करने की अनुमति देगा।

#### फ़ॉन्ट कैसे परिभाषित करें

फ़ॉन्ट को ओवरराइड करने के लिए, हम अनुशंसा करते हैं कि आप अपनी CSS को `.fast-comments, textarea` सेलेक्टर्स का उपयोग करके परिभाषित करें। उदाहरण के लिए:

[inline-code-attrs-start title = 'कस्टम एक्सटर्नल फ़ॉन्ट उदाहरण'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]