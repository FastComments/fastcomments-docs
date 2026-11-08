पोल को REST API के ज़रिए आपके अपने कोड से भी प्रबंधित किया जा सकता है, उदाहरण के लिए अपने बैकएंड से किसी कमेंट में पोल जोड़ने, पोल बंद करने, या वोट एक्सपोर्ट करने के लिए:

- [Poll Structure](/guide-api.html#poll-structure) पोल ऑब्जेक्ट और उसकी सीमाओं का वर्णन करता है।
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), और [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` किसी कमेंट पर पोल को पढ़ते, बदलते, संपादित करते और हटाते हैं।
- [PollVote Structure](/guide-api.html#poll-vote-structure) एक वोट का वर्णन करता है।
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), और [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` वोट को सूचीबद्ध करते हैं, डालते और हटाते हैं।

`POST /api/v1/comments` को `poll` पास करके एक पोल को उसके कमेंट के साथ भी बनाया जा सकता है।

API विजेट के समान सेटिंग्स का पालन करता है: पोल सक्षम होने चाहिए, और लॉग इन उपयोगकर्ता के बिना वोट अस्वीकार कर दिए जाते हैं जब तक कि अनाम पोल वोट की अनुमति न हो। अनाम पोल पर व्यक्तिगत वोट API के ज़रिए नहीं पढ़े जा सकते।