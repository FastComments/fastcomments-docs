FastComments मॉडरेटर्स और एडमिनिस्ट्रेटर्स के लिए दैनिक, साप्ताहिक या मासिक ईमेल डाइजेस्ट का समर्थन करता है।

इसकी आवृत्ति <a href="" target="_blank">यहाँ</a> से कॉन्फ़िगर की जा सकती है।

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='नोटिफिकेशन संपादन फ़ॉर्म जहाँ डाइजेस्ट दैनिक, साप्ताहिक या मासिक आने के लिए सेट है'; title='डाइजेस्ट आवृत्ति कॉन्फ़िगर करना' app-screenshot-end]

आपकी टिप्पणियों के समग्र आँकड़े शामिल करने के साथ-साथ, यह समीक्षा की आवश्यकता वाली तीन सबसे हालिया टिप्पणियाँ भी सूचीबद्ध करेगा।

इनमें से प्रत्येक टिप्पणी के लिए, सीधे मैजिक लिंक उपलब्ध कराए जाते हैं:
- टिप्पणी को स्वीकृत करने के लिए।
- टिप्पणी को समीक्षित चिह्नित करने और उत्तर पृष्ठ पर जाने के लिए।
- टिप्पणी को स्पैम चिह्नित करने के लिए।

प्रत्येक टिप्पणी के लिए ये लिंक आपको स्वचालित रूप से प्रमाणित करेंगे और आपके ईमेल से कार्रवाई करेंगे।

इसके अतिरिक्त, डाइजेस्ट में एक Moderate Comments बटन होता है जो वही प्रमाणीकरण करेगा और आपको
Moderate Comments पृष्ठ पर ले जाएगा।

कृपया ध्यान दें कि ये मैजिक लिंक कुछ समय बाद समाप्त हो जाते हैं।

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='मासिक डाइजेस्ट ईमेल जिसमें टिप्पणी आँकड़े और समीक्षा की आवश्यकता वाली तीन टिप्पणियाँ हैं, प्रत्येक में स्वीकृत, उत्तर और स्पैम लिंक हैं'; title='डाइजेस्ट ईमेल' app-screenshot-end]

#### नोटिफिकेशन प्रकार

FastComments मॉडरेटर्स और एडमिनिस्ट्रेटर्स को कई प्रकार के ईमेल भेजता है। यदि चाहें, तो `Comment Reply` नोटिफिकेशन से ऑप्ट आउट करना संभव है, साथ ही
ऊपर दिखाए गए `Edit Notifications` पृष्ठ में उपयुक्त विकल्प चुनकर `New Comment` नोटिफिकेशन प्राप्त करना जारी रखना संभव है।