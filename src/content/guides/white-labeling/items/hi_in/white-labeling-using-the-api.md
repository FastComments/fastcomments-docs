API का उपयोग करके एक व्हाइट लेबल्ड टेनेंट बनाने के लिए, हमें निम्नलिखित करना होगा:

1. टेनेंट बनाने के लिए [Tenants API](/guide-api.html#tenant-structure) को कॉल करें।
2. टेनेंट के लिए एक या अधिक पैकेज बनाने के लिए [TenantPackages API](/guide-api.html#tenant-package-structure) को कॉल करें।
3. यह परिभाषित करने के लिए [Tenants API](/guide-api.html#tenant-patch) को कॉल करें कि टेनेंट पर कौन सा पैकेज सक्रिय है।
4. टेनेंट में व्यवस्थापक उपयोगकर्ताओं को जोड़ने के लिए [TenantUsers API](/guide-api.html#tenant-user-structure) को कॉल करें।
5. टेनेंट में मॉडरेटर जोड़ने और आमंत्रित करने के लिए [Moderators API](/guide-api.html#moderator-structure) को कॉल करें।
6. वैकल्पिक रूप से, [SSO सेटअप करें](/guide-customizations-and-configuration.html#sso)।

यह सब ट्रायल अवधि के भीतर किया जा सकता है।