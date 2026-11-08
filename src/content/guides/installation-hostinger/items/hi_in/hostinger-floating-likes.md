FastComments Hostinger के लिए Page Reacts (aka Floating Like button) विजेट को भी सपोर्ट करता है।

आप इसे इस पेज के निचले दाएँ कोने में एक्शन में देख सकते हैं!

### नोट!

ये निर्देश Hostinger Site Builder के लिए हैं। यदि आप Hostinger *WordPress* का उपयोग कर रहे हैं, तो बस नीचे दिया गया कोड लें और इसे अपनी WordPress साइट में [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/) के उपयोग से जोड़ें, जो आपकी साइट में छोटे कोड स्निपेट जोड़ने के लिए एक मुफ्त और आसान प्लगइन है।

1. सबसे पहले, कोड लें:

[inline-code-attrs-start title = 'Hostinger Floating Likes कोड'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. फिर, Hostinger में, साइट बिल्डर खोलें।
3. निचले बाएँ कोने में Website Settings में जाएँ।
4. Integrations चुनें।
5. नए कोड को `Custom code` फ़ील्ड के *अंत* में जोड़ें, और अपनी साइट प्रकाशित करें।
6. आप विजेट को प्रीव्यू मोड में नहीं देखेंगे, लेकिन यह साइट के प्रकाशित संस्करण पर दिखाई देगा।