कॉन्फ़िग तीन जगहों से आती है। बाद के स्रोत प्राथमिकता रखते हैं:

1. `_config.yml` में `fastcomments:` कुंजी के अंतर्गत **ग्लोबल डिफ़ॉल्ट**।
2. **पेज संदर्भ**, पेज-स्कोप्ड विजेट्स के लिए स्वचालित रूप से प्राप्त (नीचे देखें)।
3. टैग पर ही लिखे गए **टैग एट्रिब्यूट्स**।

इसलिए टैग पर दिया गया `url_id` पेज-प्राप्त मान को ओवरराइड करता है, जो किसी भी ग्लोबल डिफ़ॉल्ट को ओवरराइड करता है।

### एट्रिब्यूट सिंटैक्स

एट्रिब्यूट्स `snake_case` में `key=value` जोड़े होते हैं:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **कोट किए गए** मान (`"..."` या `'...'`) शाब्दिक स्ट्रिंग होते हैं।
- **बिना कोट के** `true`/`false` बूलियन बन जाते हैं, और संख्याएँ नंबर बन जाती हैं।
- **बिना कोट के** अन्य कुछ भी पेज संदर्भ से Liquid वेरिएबल के रूप में रिज़ॉल्व होता है, जैसे
  `url_id=page.slug`। (Liquid टैग के एट्रिब्यूट्स के अंदर `{% raw %}\{{ ... }}{% endraw %}` को विस्तारित नहीं करता,
  इसलिए `"{% raw %}\{{ page.slug }}{% endraw %}"` के बजाय बिना कोट वाले `page.slug` रूप का उपयोग करें।)

Snake_case एट्रिब्यूट और कॉन्फ़िग कुंजियाँ स्वचालित रूप से FastComments द्वारा अपेक्षित camelCase कुंजियों में मैप हो जाती हैं
(`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground`, आदि)।
[विजेट कॉन्फ़िगरेशन](https://docs.fastcomments.com/guide-customizations-and-configuration.html) का कोई भी अन्य विकल्प
उसी तरह सीधे पास हो जाता है।

### पेज-प्राप्त मान

पेज-स्कोप्ड विजेट्स (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`) के लिए ये स्वचालित रूप से वर्तमान पेज से भर जाते हैं,
जब तक कि आप स्वयं उन्हें सेट न करें:

- `url_id` ← `page.url` (विज़िटिंग डोमेन से स्वतंत्र एक स्थिर पहचानकर्ता)
- `url` ← `site.url` + `page.url` (केवल तब जब `url` `_config.yml` में सेट हो)
- `page_title` ← `page.title`

साइट-व्यापी विजेट्स (हाल की टिप्पणियाँ/चर्चाएँ, शीर्ष पेज, समीक्षा सारांश, उपयोगकर्ता गतिविधि फ़ीड,
बल्क काउंट) किसी पेज से बंधे नहीं होते और ये मान प्राप्त नहीं करते।

### EU डेटा रेज़िडेंसी

EU ग्राहक `region: eu` जोड़ते हैं, या तो ग्लोबली:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

या प्रति टैग: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`। विजेट्स तब EU CDN से लोड होते हैं।