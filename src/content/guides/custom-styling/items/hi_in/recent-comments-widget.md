Recent Comments Widget आपकी साइट भर में या किसी विशेष पेज के लिए सबसे हालिया टिप्पणियों की सूची प्रदर्शित करता है। इसमें एक शीर्षक, गोल अवतार, टिप्पणी प्रीव्यू, क्लिक करने योग्य तारीखें (जो टिप्पणी तक डीप-लिंक करती हैं) और स्वचालित डार्क मोड डिटेक्शन शामिल है।

## बेसिक इंस्टॉलेशन

[inline-code-attrs-start title = 'Recent Comments Widget इंस्टॉलेशन'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-comments-v2.min.js"></script>
<div id="fastcomments-widget-recent-comments"></div>
<script>
    FastCommentsRecentCommentsV2(document.getElementById('fastcomments-widget-recent-comments'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## कॉन्फ़िगरेशन विकल्प

- **tenantId** (आवश्यक): आपकी FastComments tenant ID
- **urlId** (वैकल्पिक): एक ही पेज तक फ़िल्टर करें। सभी पेजों के लिए null छोड़ दें
- **count** (वैकल्पिक): दिखाने के लिए टिप्पणियों की संख्या। डिफ़ॉल्ट `10` है
- **hasDarkBackground** (वैकल्पिक): डार्क मोड स्टाइलिंग लागू करें। सेट न होने पर पेज की पृष्ठभूमि से स्वतः पहचाना जाता है

## विजेट संरचना

विजेट निम्नलिखित HTML संरचना के साथ रेंडर होता है:

[inline-code-attrs-start title = 'Recent Comments Widget HTML संरचना'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rc2">
    <div class="fc-rc2-heading">Recent Comments</div>
    <div class="fc-rc2-list">
        <div class="fc-rc2-card">
            <div class="fc-rc2-header">
                <img class="fc-rc2-avatar" src="..." alt="Avatar" />
                <div class="fc-rc2-meta">
                    <span class="fc-rc2-name">Username</span>
                    <a class="fc-rc2-date" href="...">2 hours ago</a>
                </div>
            </div>
            <div class="fc-rc2-body">Comment preview...</div>
            <a class="fc-rc2-page-link" href="...">Page Title</a>
        </div>
    </div>
</div>
[inline-code-end]

## डिफ़ॉल्ट CSS संदर्भ

[inline-code-attrs-start title = 'Recent Comments Widget डिफ़ॉल्ट CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rc2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rc2-card { padding: 14px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rc2-card:last-child { border-bottom: none; }
.fc-rc2-header { display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }
.fc-rc2-avatar { width: 32px; height: 32px; border-radius: 50%; object-fit: cover; }
.fc-rc2-name { font-size: 13px; font-weight: 600; }
.fc-rc2-date { font-size: 11.5px; color: #999; text-decoration: none; }
.fc-rc2-body { font-size: 14px; line-height: 1.55; color: #444; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
.fc-rc2-page-link { display: inline-block; margin-top: 6px; font-size: 12px; color: #777; text-decoration: none; }
.fc-rc2-page-link:hover { color: #0066cc; text-decoration: underline; }
[inline-code-end]

## कस्टमाइज़ेशन उदाहरण

### अवतार का आकार बदलें

[inline-code-attrs-start title = 'कस्टम अवतार आकार'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2-avatar {
    width: 40px !important;
    height: 40px !important;
}
[inline-code-end]

### टिप्पणी टेक्स्ट की अधिक पंक्तियाँ दिखाएँ

[inline-code-attrs-start title = 'अधिक टिप्पणी पंक्तियाँ दिखाएँ'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2-body {
    -webkit-line-clamp: 5 !important;
}
[inline-code-end]

### कंटेनर बॉर्डर हटाएँ

[inline-code-attrs-start title = 'कंटेनर बॉर्डर हटाएँ'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rc2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]