[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

हमने देखा कि `urlId` वह पेज या आर्टिकल आईडी है जिससे कमेंट्स जुड़े होते हैं।

साथ ही, याद दिलाने के लिए, यदि परिभाषित नहीं किया गया है तो `urlId` डिफ़ॉल्ट रूप से वर्तमान पेज URL पर सेट हो जाता है।

लेकिन SPAs, यानी Single-Page-Applications का क्या, जहाँ कमेंट्स जिस पेज या कंटेंट से जुड़े हैं वह बिना पेज रीलोड के डायनामिक रूप से बदल जाता है?

#### Angular, React, Vue, आदि

हमारी लाइब्रेरीज़ जैसे Angular और React के साथ, विजेट को पास की गई `urlId` प्रॉपर्टी को अपडेट करने से ही कमेंट विजेट रिफ्रेश हो जाता है। आप इसे React ऐप में एक्शन में देख सकते हैं, उदाहरण के लिए, <a href="https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx#L39" target="_blank">यहाँ</a>।

#### VanillaJS

यदि आप VanillaJS लाइब्रेरी का उपयोग कर रहे हैं तो यह थोड़ा अधिक जटिल है, क्योंकि Angular या React जैसा कोई फ्रेमवर्क नहीं होता जो डेटा बाइंडिंग या स्टेट प्रोपेगेशन को संभालता हो।

जब आप VanillaJS विजेट को इंस्टेंटिएट करते हैं, तो यह कुछ फ़ंक्शंस रिटर्न करता है जिन्हें इसे अपडेट करने के लिए कॉल किया जा सकता है।

यहाँ एक फंक्शनल उदाहरण है जहाँ हम पेज हैश बदलते हैं और कमेंट विजेट को अपडेट करते हैं:

[inline-code-attrs-start title = 'पेज हैश बदलने का उदाहरण'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // हम url भी अपडेट करते हैं, ताकि नोटिफिकेशन सही पेज से लिंक हो सकें
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]