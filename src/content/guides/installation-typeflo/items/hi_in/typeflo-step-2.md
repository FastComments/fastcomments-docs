Custom Code टैब के **Footer** सेक्शन में, निम्नलिखित कोड पेस्ट करें:

[inline-code-attrs-start title = 'Typeflo.io लाइव कमेंटिंग कोड स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Footer सेक्शन में कोड पेस्ट करें</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Footer सेक्शन में FastComments कोड पेस्ट करें" />
</div>

कोड पेस्ट करने के बाद, अपने बदलाव लागू करने के लिए **Save** बटन पर क्लिक करें।

नोट: इस कोड में वह लॉजिक शामिल है जो आपके Typeflo.io ब्लॉग पोस्ट पर FastComments विजेट को सबसे उपयुक्त स्थान पर डायनामिक रूप से रखता है। अन्य कोड स्निपेट Typeflo.io के लेआउट के साथ ठीक से काम नहीं करेंगे।

साइन अप करने के बाद `'demo'` को अपनी वास्तविक FastComments tenant ID से बदलना न भूलें। यदि आप FastComments.com में लॉग इन हैं, तो यह पहले से ही बदला हुआ होना चाहिए।