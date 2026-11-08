#### Sakai

LTI Advantage वाले रिलीज़ में Sakai LTI 1.3 Dynamic Registration को सपोर्ट करता है। Administration Workspace से:

1. Sakai एडमिन के रूप में साइन इन करें और **Administration Workspace** खोलें।
2. **External Tools** > **Install LTI 1.3 Tool** चुनें।
3. FastComments रजिस्ट्रेशन URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">इसे यहाँ से प्राप्त करें</a>) पेस्ट करें और सबमिट करें।
4. हैंडशेक पूरा होने पर टूल को अप्रूव करें।

इसके बाद टूल **External Tools** के अंतर्गत दिखाई देता है और साइटों के मेंटेनर इसे साइटों में जोड़ सकते हैं।

#### Schoology

Schoology Enterprise इंस्टेंस LTI 1.3 को सपोर्ट करते हैं लेकिन Dynamic Registration की उपलब्धता डिप्लॉयमेंट के अनुसार अलग-अलग होती है। अपने Schoology अकाउंट मैनेजर से जाँच करें।

यदि आपके Schoology इंस्टेंस पर Dynamic Registration उपलब्ध नहीं है, तो आपको इन एंडपॉइंट्स का उपयोग करके इंटीग्रेशन को मैन्युअल रूप से कॉन्फ़िगर करना होगा:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Schoology द्वारा आपको Client ID और Deployment ID देने के बाद, अपने टेनेंट पर कॉन्फ़िगरेशन रजिस्टर करने के लिए FastComments सपोर्ट से संपर्क करें।

#### अन्य LTI 1.3 प्लेटफ़ॉर्म

IMS LTI 1.3 Advantage स्पेक का पालन करने वाला कोई भी LMS उसी रजिस्ट्रेशन URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">इसे यहाँ से प्राप्त करें</a>) के साथ काम करना चाहिए। "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", या इसी तरह के लेबल वाली सेटिंग खोजें।

यदि आपका प्लेटफ़ॉर्म केवल मैन्युअल LTI 1.3 सेटअप को सपोर्ट करता है, तो ऊपर Schoology सेक्शन में दिए गए चार एंडपॉइंट्स का उपयोग करें और पूरा करने के लिए सपोर्ट से संपर्क करें।