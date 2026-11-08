जब यूज़र प्रोफ़ाइल आपकी साइट के संदर्भ में (कमेंट विजेट के माध्यम से) खोली जाती है, तो आपके FastComments विजेट पर लागू की गई कोई भी कस्टम CSS स्टाइल स्वचालित रूप से प्रोफ़ाइल मोडल में इंजेक्ट हो जाती है।

### यह कैसे काम करता है

जब कोई यूज़र आपके कमेंट विजेट से प्रोफ़ाइल लिंक पर क्लिक करता है, तो `.fast-comments-profile` क्लास के साथ एक प्रोफ़ाइल मोडल खुलता है। आपके विजेट की कस्टम CSS स्वचालित रूप से प्रोफ़ाइल व्यू में इंजेक्ट हो जाती है। यदि आप पहले से अपने कमेंट विजेट को स्टाइल कर चुके हैं, तो वे स्टाइल प्रोफ़ाइल पर भी लागू होंगे।

### CSS क्लासेस

FastComments प्रोफ़ाइल क्लास-आधारित CSS आर्किटेक्चर का उपयोग करती हैं। यह CSS कस्टम प्रॉपर्टीज़ का उपयोग नहीं करता।

मुख्य प्रोफ़ाइल पेज रूट कंटेनर के रूप में `.user-profile` का उपयोग करता है। हेडर सेक्शन `.profile-header` है, जिसमें बैकग्राउंड इमेज के लिए `.profile-header-background` है। प्रोफ़ाइल कंटेंट `.profile-content` में स्थित होता है।

अवतार के लिए `.profile-avatar` और `.profile-avatar-wrapper` का उपयोग होता है। यूज़र का नाम `.profile-name` है और बायो टेक्स्ट `.profile-bio` है। आंकड़े `.profile-stats` में होते हैं, जिसमें प्रत्येक आंकड़े के लिए `.stat` का उपयोग होता है।

सोशल लिंक `.profile-social-links` में होते हैं, जिसमें प्रत्येक लिंक `.social-link` होता है। बैज के लिए `.profile-badges` और `.badge` का उपयोग होता है। बैज प्रोग्रेस बार के लिए `.progress-outer` और `.progress-bar` का उपयोग होता है।

टैब के लिए कंटेनर `.profile-tabs`, प्रत्येक टैब के लिए `.tab`, और चयनित टैब के लिए `.tab.active` का उपयोग होता है। टैब कंटेंट के लिए `.tab-body` और `.tab-body.active` का उपयोग होता है। टैब पर नोटिफिकेशन काउंट के लिए `.tab .count` का उपयोग होता है।

नोटिफिकेशन के लिए `.notification` और DM बातचीत के लिए `.conversation` का उपयोग होता है। ऑनलाइन स्थिति `.activity-indicator` है, जिसमें सक्रिय अवस्था के लिए `.activity-indicator.online` है। अपठित काउंटर के लिए `.unread-count` का उपयोग होता है।

प्रोफ़ाइल मोडल कंटेनर `.fast-comments-profile` है, जिसमें क्लोज़ बटन के लिए `.fast-comments-profile-close` है।

### डार्क मोड

डार्क मोड `.user-profile` पर `.dark` क्लास मॉडिफ़ायर का उपयोग करता है।

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### उदाहरण

**हेडर:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**बैज:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**टैब:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**मोडल:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```