पेज रिएक्ट्स (Page Reacts) के लिए हमें दो चीज़ों पर निर्णय लेना होता है:

- कौन सी रिएक्शन इमेज का उपयोग करना है।
- प्रत्येक रिएक्शन को नाम देने के लिए एक छोटा `id`।

वैकल्पिक रूप से:

- आप चयनित/अचयनित रिएक्ट्स के लिए अलग वैकल्पिक इमेज भी परिभाषित कर सकते हैं।
- आप तय कर सकते हैं कि किसी रिएक्शन पर माउस घुमाने पर उसे रिएक्ट करने वाले उपयोगकर्ताओं की सूची दिखानी है या नहीं।

[inline-code-attrs-start title = 'पेज रिएक्ट्स कोड उदाहरण'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.fcConfigs = [{
        target: '#page-reacts-example',
        tenantId: 'demo',
        pageReactConfig: {
            showUsers: true,
            reacts: [
                {id: 'droll', src: 'https://docs.fastcomments.com/images/emojis/droll.png'},
                {id: 'he', src: 'https://docs.fastcomments.com/images/emojis/heart-eyes.png'},
                {id: 'laugh', src: 'https://docs.fastcomments.com/images/emojis/laugh.png'},
                {id: 'puke', src: 'https://docs.fastcomments.com/images/emojis/puke.png', selectedSrc: 'https://docs.fastcomments.com/images/emojis/puke-bw.png' },
                {id: 'rofl', src: 'https://docs.fastcomments.com/images/emojis/rofl.png' },
            ]
        }
    }];
</script>
[inline-code-end]

React, Angular आदि जैसी फ्रंटएंड लाइब्रेरीज़ के लिए कॉन्फ़िगरेशन समान है।