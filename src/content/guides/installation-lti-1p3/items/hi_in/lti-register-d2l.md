D2L Brightspace LTI Advantage एडमिन इंटरफ़ेस के माध्यम से Dynamic Registration उपलब्ध कराता है। आपको एडमिनिस्ट्रेटर एक्सेस की आवश्यकता होगी।

#### पंजीकरण स्क्रीन खोलें

1. एडमिन के रूप में अपने Brightspace इंस्टेंस में साइन इन करें।
2. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** पर जाएँ।
3. **Register Tool** पर क्लिक करें। (सीधा URL है: `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`।)

#### URL पेस्ट करें

आपको एक पंजीकरण फ़ॉर्म दिखाई देगा। मुख्य फ़ील्ड है **Tool initiation registration endpoint** (कुछ Brightspace संस्करणों में इसे "Tool Initiation Registration URL" लेबल किया गया है)।

FastComments पंजीकरण URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">इसे यहाँ से प्राप्त करें</a>) को उस फ़ील्ड में पेस्ट करें। अन्य फ़ील्ड खाली छोड़ दें - वे पंजीकरण हैंडशेक के दौरान FastComments द्वारा स्वतः भर दिए जाते हैं।

**Register** पर क्लिक करें।

#### टूल को स्वीकृति दें

Brightspace एक पॉपअप खोलता है जो FastComments से संवाद करता है, कुंजियों का आदान-प्रदान करता है, और एक पुष्टि स्क्रीन दिखाता है। पंजीकरण पूरा होने पर पॉपअप स्वयं बंद हो जाता है।

नया टूल आपकी LTI Advantage टूल सूची में दिखाई देता है। डिफ़ॉल्ट रूप से Brightspace नए टूल को **disabled** के रूप में चिह्नित करता है - टॉगल को **enabled** में बदलें ताकि आपके कोर्स इसका उपयोग कर सकें।

#### एक Deployment जोड़ें

Brightspace में, LTI टूल को कोर्स में उपयोग किए जाने से पहले एक **deployment** की आवश्यकता होती है:

1. नए पंजीकृत FastComments टूल को खोलें।
2. **View Deployments** > **New Deployment** पर क्लिक करें।
3. Deployment को एक नाम दें (जैसे "FastComments - All Courses"), उन org यूनिटों को चुनें जहाँ यह उपलब्ध होना चाहिए, और सहेजें।

इस deployment के माध्यम से पहले लॉन्च के बाद, FastComments `deployment_id` को अपने कॉन्फ़िगरेशन रिकॉर्ड से जोड़ देता है - उसी क्लाइंट के अंतर्गत किसी भिन्न deployment से बाद के लॉन्च अस्वीकार कर दिए जाएँगे जब तक कि आप पुनः पंजीकरण न करें।