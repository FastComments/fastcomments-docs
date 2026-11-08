डिफ़ॉल्ट रूप से, FastComments उपयोगकर्ता का नाम वैसे दिखाएगा जैसा उन्होंने दर्ज किया था, या जैसे वह SSO के माध्यम से हमें भेजा गया था।

हालाँकि, उपयोगकर्ता के नाम को छिपाना या किसी अलग तरीके से दिखाना बेहतर हो सकता है। उदाहरण के लिए, यदि उपयोगकर्ता का नाम Allen Rex है, तो शायद
आप केवल "Allen R." दिखाना चाहें।

यह Widget Customization UI में, `Commenter Name Format` नामक सेटिंग के तहत, बिना कोड के किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Commenter Name Format ड्रॉपडाउन खुला है जिसमें Capitalize, Last Initial और All Initials जैसे विकल्प हैं'; title='नाम फ़ॉर्मैट बदलें' app-screenshot-end]

उपलब्ध फ़ॉर्मैट हैं:

- Capitalize (उदाहरण उपयोगकर्ता को Example User के रूप में दिखाएं)
- Last Initial (Example User को Example U. के रूप में दिखाएं)
- All Initials (Example User को E. U. के रूप में दिखाएं)
- "Anonymous" दिखाएं

इसे बदलने का प्रभाव तुरंत होता है। उपयोगकर्ता खुद के लिए कमेंट क्षेत्र के शीर्ष पर अपना पूरा उपयोगकर्ता नाम देखेंगे, लेकिन उनके कमेंट में
संशोधित उपयोगकर्ता नाम दिखाया जाएगा।

उपयोगकर्ताओं की सुरक्षा के लिए उपयोगकर्ता नाम सर्वर-साइड पर छिपाए जाते हैं।