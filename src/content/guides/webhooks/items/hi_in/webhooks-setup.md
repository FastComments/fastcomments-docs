`localhost` के लिए वही चरण अपनाएँ जो आप production के लिए करते हैं। सुनिश्चित करें कि आपके पास production domains और API Secrets सेटअप हैं।

सबसे पहले, [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks) पर जाएँ। इसे Manage Data -> Webhooks के माध्यम से एक्सेस किया जा सकता है।

यह पेज आपके खाते के प्रत्येक webhook की सूची दिखाता है:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks एडमिन पेज जो प्रत्येक webhook की URL, इवेंट, डोमेन, मेथड, स्थिति और कतार में मौजूद इवेंट की संख्या के साथ सूची दिखाता है'; title='Webhooks सूची'; cacheBuster = 'v4' app-screenshot-end]

एक जोड़ने के लिए **New Webhook** पर क्लिक करें। प्रत्येक webhook में एक URL, एक कमेंट इवेंट (created, updated या deleted), एक डोमेन, और एक HTTP मेथड होती है:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='नए webhook का फ़ॉर्म जिसमें URL, इवेंट, डोमेन और HTTP मेथड फ़ील्ड के साथ Send Test Payload भी है'; title='नया Webhook'; cacheBuster = 'v4' app-screenshot-end]

प्रत्येक webhook स्वतंत्र रूप से डिलीवर किया जाता है। आप एक ही इवेंट को कई endpoints पर भेज सकते हैं, और **All Domains** के लिए स्कोप किया गया webhook हर डोमेन से कमेंट प्राप्त करता है, भले ही उसी इवेंट के लिए कोई डोमेन-विशिष्ट webhook मौजूद हो।
एक ही URL, इवेंट और डोमेन दो बार नहीं जोड़ा जा सकता।

सेव करने से पहले, यह जाँचने के लिए **Send Test Payload** पर क्लिक करें कि endpoint एक साइन किया गया रिक्वेस्ट स्वीकार करता है या नहीं। विवरण के लिए अगला सेक्शन, "Testing", देखें।

सूची से आप किसी webhook को एडिट, डिसेबल, फिर से इनेबल या डिलीट कर सकते हैं। डिसेबल करने पर कतार में मौजूद इवेंट तब तक बने रहते हैं जब तक webhook फिर से इनेबल नहीं किया जाता; डिलीट करने पर वे हटा दिए जाते हैं।

Webhooks को API के माध्यम से भी बनाया जा सकता है, उदाहरण के लिए Zapier द्वारा। वे उसी सूची में स्रोत **API** के साथ दिखाई देते हैं। API के माध्यम से Webhooks प्रबंधित करना देखें।