FastComments में SAML कॉन्फ़िगर करने के बाद, आपको अपने आइडेंटिटी प्रोवाइडर में FastComments को एक Service Provider के रूप में सेट अप करना होगा।

### सामान्य IdP कॉन्फ़िगरेशन

अधिकांश आइडेंटिटी प्रोवाइडर्स को FastComments को SAML एप्लिकेशन के रूप में जोड़ने के लिए निम्नलिखित जानकारी की आवश्यकता होती है:

#### आवश्यक Service Provider जानकारी

ये मान स्वचालित रूप से जेनरेट होते हैं और आपके FastComments SAML कॉन्फ़िगरेशन पेज पर दिखाए जाते हैं:

**SP Entity ID / Audience**
- फ़ॉर्मैट: `https://fastcomments.com/saml/{your-tenant-id}`
- यह आपके FastComments इंस्टेंस की अद्वितीय पहचान करता है

**Assertion Consumer Service (ACS) URL**
- फ़ॉर्मैट: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- यहाँ आपका IdP प्रमाणीकरण के बाद SAML रिस्पॉन्स भेजता है

**SP Metadata URL** *(यदि आपका IdP समर्थन करता है)*
- फ़ॉर्मैट: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- XML फ़ॉर्मैट में संपूर्ण SAML कॉन्फ़िगरेशन प्रदान करता है

**SAML Login URL**
- फ़ॉर्मैट: `https://fastcomments.com/saml/login/{your-tenant-id}`
- SAML प्रमाणीकरण शुरू करने के लिए सीधा लिंक

### आवश्यक SAML एट्रिब्यूट्स

अपने आइडेंटिटी प्रोवाइडर को SAML रिस्पॉन्स के साथ ये एट्रिब्यूट्स भेजने के लिए कॉन्फ़िगर करें:

#### आवश्यक एट्रिब्यूट्स

**ईमेल पता** *(आवश्यक)*
- **एट्रिब्यूट नाम**: `email`, `emailAddress`, या `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **उद्देश्य**: अद्वितीय यूज़र पहचान और नोटिफिकेशन
- **फ़ॉर्मैट**: वैध ईमेल पता

#### वैकल्पिक एट्रिब्यूट्स

**पहला नाम**
- **एट्रिब्यूट नाम**: `firstName`, `givenName`, या `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **उद्देश्य**: यूज़र डिस्प्ले नाम

**अंतिम नाम**
- **एट्रिब्यूट नाम**: `lastName`, `surname`, या `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **उद्देश्य**: यूज़र डिस्प्ले नाम

**भूमिकाएँ (Roles)** *(एक्सेस कंट्रोल के लिए महत्वपूर्ण)*
- **एट्रिब्यूट नाम**: `roles`, `groups`, `memberOf`, या कस्टम एट्रिब्यूट नाम
- **उद्देश्य**: FastComments भूमिका असाइनमेंट और अनुमतियाँ
- **फ़ॉर्मैट**: भूमिका स्ट्रिंग्स की ऐरे या कॉमा से अलग किए गए मान

### सामान्य आइडेंटिटी प्रोवाइडर कॉन्फ़िगरेशन

#### Microsoft Azure AD

1. **Enterprise एप्लिकेशन जोड़ें**
   - "FastComments" खोजें या एक कस्टम SAML एप्लिकेशन बनाएँ
   - FastComments द्वारा प्रदान की गई SP जानकारी का उपयोग करें

2. **एट्रिब्यूट्स कॉन्फ़िगर करें**
   - ईमेल: `user.mail` या `user.userprincipalname`
   - पहला नाम: `user.givenname`
   - अंतिम नाम: `user.surname`
   - भूमिकाएँ: `user.assignedroles` या डायरेक्टरी समूह

#### Okta

1. **SAML एप्लिकेशन बनाएँ**
   - "Create New App" का उपयोग करें और SAML 2.0 चुनें
   - FastComments SP जानकारी के साथ कॉन्फ़िगर करें

2. **Attribute Statements**
   - ईमेल: `user.email`
   - पहला नाम: `user.firstName`
   - अंतिम नाम: `user.lastName`
   - भूमिकाएँ: `user.groups` या कस्टम एट्रिब्यूट्स

#### Google Workspace

1. **SAML एप्लिकेशन जोड़ें**
   - Apps > Web and mobile apps > Add App > Add custom SAML app पर जाएँ
   - FastComments SP जानकारी के साथ कॉन्फ़िगर करें

2. **एट्रिब्यूट मैपिंग**
   - ईमेल: प्राथमिक ईमेल
   - पहला नाम: पहला नाम
   - अंतिम नाम: अंतिम नाम
   - भूमिकाएँ: समूह या कस्टम एट्रिब्यूट्स

#### Active Directory Federation Services (ADFS)

1. **Relying Party Trust जोड़ें**
   - FastComments metadata URL या मैन्युअल कॉन्फ़िगरेशन का उपयोग करें
   - दी गई SP जानकारी के अनुसार कॉन्फ़िगर करें

2. **Claim Rules**
   - ईमेल: Email Address claim
   - नाम: Name ID claim
   - भूमिकाएँ: समूह सदस्यता या कस्टम claims

### एट्रिब्यूट नाम लचीलापन

FastComments विभिन्न IdP कॉन्फ़िगरेशन को समायोजित करने के लिए कई एट्रिब्यूट नामों से भूमिका जानकारी स्वीकार करता है:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

यह लचीलापन विशिष्ट एट्रिब्यूट नामकरण परंपराओं की आवश्यकता के बिना विभिन्न आइडेंटिटी प्रोवाइडर्स के साथ संगतता सुनिश्चित करता है।

### अपने कॉन्फ़िगरेशन का परीक्षण

अपने आइडेंटिटी प्रोवाइडर को कॉन्फ़िगर करने के बाद:

1. IdP कॉन्फ़िगरेशन सहेजें
2. एक समर्पित टेस्ट यूज़र अकाउंट के साथ परीक्षण करें
3. सत्यापित करें कि एट्रिब्यूट्स सही ढंग से भेजे जा रहे हैं
4. जाँचें कि भूमिकाएँ ठीक से मैप हो रही हैं
5. सुनिश्चित करें कि प्रमाणीकरण प्रवाह सफलतापूर्वक पूरा होता है

अधिकांश आइडेंटिटी प्रोवाइडर्स प्रोडक्शन यूज़र्स में तैनात करने से पहले कॉन्फ़िगरेशन को मान्य करने के लिए SAML परीक्षण टूल प्रदान करते हैं।