[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments विजेट खुद को लंबवत रूप से रीसाइज़ कर लेता है ताकि सभी दिखाई देने वाली कमेंट्स फिट हो जाएं। पेजिनेशन मौजूदा पेज के अंत में एक "View Next" बटन के माध्यम से होती है, क्योंकि हमने पाया है कि यह इंटरैक्शन अधिकांश उपयोगकर्ताओं को सबसे अच्छा लगता है।

हालाँकि, कुछ मामलों में इनफिनिट स्क्रॉलिंग बेहतर होती है। उदाहरण के लिए, हम इस फीचर का उपयोग अपने Stream Chat प्रोडक्ट में करते हैं।

हम **enableInfiniteScrolling** फ्लैग को true सेट करके "View Next" बटन छिपा सकते हैं और इनफिनिट स्क्रॉलिंग पर स्विच कर सकते हैं:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'इनफिनिट स्क्रॉलिंग सक्षम करना'; code-example-end]

इसके लिए कस्टम CSS जोड़ना भी आवश्यक है। स्क्रॉलिंग सक्षम करने के लिए `.comments` सिलेक्टर के लिए कस्टम CSS जोड़ें, उदाहरण के लिए:

[inline-code-attrs-start title = 'इनफिनिट स्क्रॉलिंग सक्षम करें'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

एक पूर्ण कार्यशील उदाहरण इस प्रकार होगा:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'इनफिनिट स्क्रॉलिंग सक्षम करना'; code-example-end]

उपरोक्त उदाहरण में हम `customCSS` प्रॉपर्टी का उपयोग करते हैं, हालाँकि परफॉर्मेंस कारणों से सुझाव दिया जाता है कि इसके बजाय Widget Configuration UI का उपयोग किया जाए। [कस्टम CSS दस्तावेज़ीकरण देखें।](/guide-customizations-and-configuration.html#custom-css)