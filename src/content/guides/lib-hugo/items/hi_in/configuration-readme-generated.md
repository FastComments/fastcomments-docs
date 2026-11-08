सभी FastComments विजेट विकल्प `hugo.toml` में `[params.fastcomments]` के अंतर्गत सेट किए जाते हैं, और प्रति पृष्ठ front matter में `[fastcomments]` के अंतर्गत ओवरराइड किए जा सकते हैं। प्राथमिकता, निम्नतम से अधिकतम: साइट params, पृष्ठ front matter, shortcode पैरामीटर।

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

जब न `url` और न ही `urlId` प्रदान किया जाता है, तो `url` पृष्ठ के permalink पर डिफ़ॉल्ट हो जाता है ताकि कमेंट थ्रेड एक स्थिर URL से जुड़े रहें।

### EU डेटा रेजिडेंसी

EU ग्राहक विजेट को `cdn-eu.fastcomments.com` पर रूट करने के लिए `region = "eu"` सेट करते हैं:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### की केसिंग पर नोट

Hugo `hugo.toml` और front matter में हर की को लोअरकेस कर देता है, लेकिन FastComments विजेट को camelCase कीज़ (`tenantId`, `hasDarkBackground`) की आवश्यकता होती है। यह कंपोनेंट हर ज्ञात टॉप-लेवल विकल्प के लिए सही केसिंग स्वचालित रूप से पुनर्स्थापित करता है, इसलिए विकल्पों को उनके सामान्य camelCase रूप में लिखें। ऑब्जेक्ट मान के अंदर नेस्ट की गई कीज़ (उदाहरण के लिए `translations` मैप की कीज़, या `pageReactConfig` के फ़ील्ड) पुनर्स्थापित नहीं की जातीं। उन्हें इसके बजाय FastComments डैशबोर्ड कस्टमाइज़ेशन UI के माध्यम से कॉन्फ़िगर करें।