यहाँ समरी विजेट का एक छोटा डेमो है:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // क्वेरी पैरामीटर हटाएँ
    });
</script>

### कैशिंग

ध्यान दें कि समरी 30 सेकंड के लिए कैश की जाती है, या यदि समीक्षाओं की संख्या बड़ी हो तो पाँच मिनट के लिए। इस कारण से, आपकी समीक्षा समरी में तुरंत नहीं दिख सकती, लेकिन इससे
हमें समरी विजेट सर्व करने की लागत कम करने में मदद मिलती है।