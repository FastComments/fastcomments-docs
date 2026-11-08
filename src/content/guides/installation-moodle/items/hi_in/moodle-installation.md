#### प्लगइन डाउनलोड करें

<a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub रिपॉजिटरी</a> से नवीनतम रिलीज़ ZIP डाउनलोड करें।

#### अपनी Moodle डायरेक्टरी में एक्सट्रैक्ट करें

ZIP को अपनी Moodle इंस्टॉलेशन में एक्सट्रैक्ट करें ताकि प्लगइन `<moodle-root>/local/fastcomments` पर स्थित हो। प्लगइन डायरेक्टरी में `version.php`, `lib.php`, और अन्य प्लगइन फ़ाइलें सीधे (सबफ़ोल्डर में नेस्ट किए बिना) होनी चाहिए।

उदाहरण के लिए:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Moodle एडमिन के माध्यम से इंस्टॉल करें

साइट एडमिनिस्ट्रेटर के रूप में लॉग इन करें और **Site Administration > Notifications** पर जाएँ। Moodle नए प्लगइन का पता लगाएगा और आपको इंस्टॉलेशन चलाने के लिए प्रेरित करेगा।

#### प्लगइन कॉन्फ़िगर करें

इंस्टॉलेशन के बाद, अपनी सेटिंग्स दर्ज करने के लिए **Site Administration > Plugins > Local plugins > FastComments** पर जाएँ। प्रत्येक विकल्प के विवरण के लिए [Configuration](#moodle-configuration) अनुभाग देखें।