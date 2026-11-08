## अपने Super.so Notion लेखों में लाइव कमेंट विजेट जोड़ना

Collab Chat के अलावा, आप अपने Notion लेखों के निचले हिस्से में एक पारंपरिक कमेंट विजेट भी जोड़ सकते हैं। इससे पाठक लेख पर कमेंट कर सकते हैं और पूरे लेख पर चर्चा कर सकते हैं।

### इंस्टॉलेशन चरण

निम्नलिखित कोड कॉपी करें और इसे अपनी Super.so साइट सेटिंग्स के `Body` सेक्शन में पेस्ट करें:

[inline-code-attrs-start title = 'Super.so FastComments लाइव कमेंट विजेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // मौजूदा इंस्टेंस को साफ़ करें
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // नया टारगेट बनाएं
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // FastComments इनिशियलाइज़ करें
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // वर्तमान pathname अपडेट करें
            currentPathname = window.location.pathname;
        }

        // प्रारंभिक लोड
        load();

        // हर 500ms में बदलाव की जाँच करें
        setInterval(() => {
            // यदि pathname बदला हो तो रीलोड करें
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // यदि विजेट हटा दिया गया हो तो रीलोड करें
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // यदि कंटेनर खाली कर दिया गया हो तो रीलोड करें
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### महत्वपूर्ण नोट्स

- कमेंट विजेट आपके Notion लेखों के निचले हिस्से में दिखाई देगा
- प्रत्येक पेज को URL पाथ के आधार पर अपनी अलग कमेंट थ्रेड मिलती है
- `"demo"` को अपने FastComments अकाउंट के वास्तविक tenant ID से बदलना सुनिश्चित करें
- विजेट Super.so की डायनामिक पेज लोडिंग को स्वचालित रूप से संभालता है