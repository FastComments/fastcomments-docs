`Extension` के साथ इंटरैक्ट करना सरल है, क्योंकि हम केवल उन फ़ंक्शनों के संदर्भ परिभाषित करते हैं जिन्हें हम इनवोक होना चाहते हैं।

पिछले उदाहरण को आगे बढ़ाते हुए, मान लीजिए हम प्रत्येक कमेंट के शीर्ष पर HTML जोड़ना चाहते हैं:

[inline-code-attrs-start title = 'एक सरल एक्सटेंशन - जारी'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    extension.commentFilter = function(comment, html) {
        return `<h3>The user's name is ${comment.commenterName}!</h3>` + html;
    }
})();
[inline-code-end]

जब भी आप इस तरह HTML रिटर्न करते हैं, यह एक dom-diffing एल्गोरिद्म के माध्यम से UI में मर्ज हो जाएगा।

#### किसी कमेंट का पुनः रेंडर मैन्युअली ट्रिगर करना

हम प्रारंभिक पेज लोड का इंतज़ार कर सकते हैं और `reRenderComment` को इनवोक करके किसी कमेंट को मैन्युअली पुनः रेंडर कर सकते हैं:

[inline-code-attrs-start title = 'कमेंट का पुनः रेंडर'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    let renderCount = 0;

    extension.commentFilter = function(comment, html) {
        renderCount++;
        return `<h3>The render count is ${renderCount}!</h3>` + html;
    }

    extension.onInitialRenderComplete = function() {
        setInterval(function() {
            extension.reRenderComment(extension.commentsById[Object.keys(extension.commentsById)[0]], function renderDone() {
                console.log('Comment re-render done.');
            });
        }, 2000); // timeout not required, just an example.
    }
})();
[inline-code-end]