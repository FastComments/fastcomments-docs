BigCommerce के Page Builder के माध्यम से FastComments जोड़ना अनुशंसित नहीं है, क्योंकि तब कोड को हर वांछित पेज पर मैन्युअल रूप से जोड़ना पड़ता है।

हालाँकि, यदि यह वांछित है, तो निम्नलिखित कोड स्निपेट का उपयोग किया जाना चाहिए। BigCommerce की प्रकृति के कारण अन्य ट्यूटोरियल्स के कोड स्निपेट काम नहीं करेंगे:

[inline-code-attrs-start title = 'BigCommerce Page Builder स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]