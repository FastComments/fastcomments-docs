## उदाहरण Zaps

कुछ वर्कफ़्लो जिन्हें सेट अप करने में कुछ ही मिनट लगते हैं।

**नई टिप्पणियों के बारे में सूचित हों।** New Comment, फिर Slack "Send Channel Message" या Discord "Send Channel Message"। संदेश में टिप्पणीकर्ता का नाम, टिप्पणी का टेक्स्ट और पेज URL मैप करें। प्रति साइट अलग चैनल पर सूचना पाने के लिए डोमेन फ़िल्टर जोड़ें।

**हर टिप्पणी का लॉग रखें।** New Comment, फिर Google Sheets "Create Spreadsheet Row"। Deleted Comment को दूसरे Zap के रूप में जोड़ें जो comment id के साथ एक पंक्ति जोड़ता है, ताकि शीट ऑडिट ट्रेल के रूप में भी काम करे।

**टिप्पणी स्वीकृत होने पर लेखक को ईमेल भेजें।** Updated Comment के साथ Zapier फ़िल्टर Approved is true, फिर Gmail "Send Email"। क्योंकि Updated Comment हर बदलाव पर ट्रिगर होता है, यह फ़िल्टर ही इस Zap को केवल स्वीकृतियों पर प्रतिक्रिया देने योग्य बनाता है।

**टिप्पणीकर्ताओं को अपने CRM या मेलिंग लिस्ट में जोड़ें।** New Comment, फिर टिप्पणीकर्ता के ईमेल का उपयोग करके HubSpot "Create or Update Contact" या Mailchimp "Add or Update Subscriber"। किसी को मार्केटिंग लिस्ट में जोड़ने से पहले अपनी गोपनीयता नीति और स्थानीय कानून का सम्मान करें।

**फ़ॉर्म से टिप्पणी बनाएं।** Typeform या Google Forms "New Response", फिर अपनी साइट द्वारा प्रशंसापत्रों (testimonials) के लिए उपयोग किए जाने वाले पेज URL ID के साथ FastComments Create Comment। दिखाई देने से पहले प्रत्येक की समीक्षा करने के लिए Approved को अनचेक छोड़ दें।

**फ़ीड में घोषणाएँ पोस्ट करें।** RSS by Zapier "New Item in Feed", फिर आइटम के title, content, और link के साथ Create Feed Post।

**सदस्यों को SSO उपयोगकर्ताओं के रूप में प्रोविज़न करें।** Memberstack, Memberful, या अपना खुद का webhook, फिर "find or create" मोड में Find SSO User के बाद Create SSO User।

**रिपोर्ट की गई टिप्पणियों को एस्कलेट करें।** Updated Comment, शून्य से अधिक flag count पर फ़िल्टर किया गया, फिर comment id और मॉडरेशन पेज के लिंक के साथ Trello "Create Card" या Linear "Create Issue"।

**पेजों को लाइव होते ही प्रकाशित करें।** WordPress या Ghost "New Post", फिर पोस्ट URL के साथ Create Page, ताकि पहली टिप्पणी से पहले पेज सूचीबद्ध और प्रतिबंधित हो जाए।

**डिलीट की गई टिप्पणियों को संग्रहित करें।** Deleted Comment, फिर अनुपालन (compliance) प्रतिधारण के लिए पूरी टिप्पणी के साथ Airtable "Create Record"।