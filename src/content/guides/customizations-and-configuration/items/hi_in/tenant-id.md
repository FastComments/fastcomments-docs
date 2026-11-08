[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

आप देख सकते हैं कि कमेंट विजेट को "demo" Tenant ID के साथ इस्तेमाल किया जा सकता है, उदाहरण के लिए:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'डेमो Tenant ID'; code-example-end]

यह केवल कमेंट विजेट को आज़माने और उसके साथ खेलने के लिए है। प्रोडक्शन में, आप अपना Tenant ID इस तरह पास करेंगे:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'अपना Tenant ID परिभाषित करना'; code-example-end]

आपका Tenant ID पहले से लागू मिल जाएगा कमेंट विजेट के <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">आपके खाते के कोड स्निपेट</a> में।

आप अपना Tenant ID भी देख सकते हैं और अपनी API कुंजियाँ प्रबंधित कर सकते हैं [API क्रेडेंशियल पेज पर](https://fastcomments.com/auth/my-account/api-secret)।

इस बिंदु से आगे, यदि आप FastComments में लॉग इन हैं, तो कोड उदाहरण आपका वास्तविक Tenant ID उपयोग करेंगे (यदि आप https://fastcomments.com पर लॉग इन हैं)।