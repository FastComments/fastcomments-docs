यह मॉड्यूल तीन Drupal अनुमतियाँ जोड़ता है जिन्हें आप `People > Permissions` के अंतर्गत प्रति भूमिका (role) असाइन कर सकते हैं।

- **Administer FastComments** - `/admin/config/content/fastcomments` पर FastComments सेटिंग्स फ़ॉर्म तक पहुँच।
- **View FastComments** - कमेंटिंग विजेट देखने के लिए आवश्यक। इस अनुमति के बिना विजेट रेंडर नहीं होता।
- **Toggle FastComments** - उपयोगकर्ताओं को फ़ील्ड विजेट का उपयोग करके प्रति-एंटिटी आधार पर कमेंट सक्षम या अक्षम करने की अनुमति देता है।

डिफ़ॉल्ट रूप से, केवल `administer site configuration` अनुमति वाले उपयोगकर्ता ही FastComments सेटिंग्स बदल सकते हैं। यदि आप चाहते हैं कि आगंतुक विजेट देख सकें, तो अनाम (anonymous) और प्रमाणित (authenticated) उपयोगकर्ताओं को `View FastComments` प्रदान करें।