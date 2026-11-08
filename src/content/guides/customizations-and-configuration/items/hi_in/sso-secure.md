[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO, SSO लागू करने के तंत्र के रूप में HMAC-SHA256 एन्क्रिप्शन का उपयोग करता है। पहले हम समग्र आर्किटेक्चर को समझेंगे, उदाहरण और विस्तृत चरण प्रदान करेंगे।

समान SSO तंत्र वाले अन्य प्रदाताओं से माइग्रेट करने और अंतरों के बारे में भी कुछ दस्तावेज़ीकरण उपलब्ध है।

प्रवाह इस प्रकार दिखता है:

<div class="screenshot white-bg">
    <div class="title">Secure SSO प्रवाह</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO आरेख" />
</div>

चूंकि Secure SSO में फुल-स्टैक विकास शामिल है, Java/Spring, NodeJS/Express, और vanilla PHP में पूर्ण कार्यशील कोड उदाहरण वर्तमान में <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub पर</a> उपलब्ध हैं।

हालाँकि हम NodeJS उदाहरण में ExpressJS और Java उदाहरण में Spring का उपयोग करते हैं, FastComments SSO लागू करने के लिए इन रन-टाइम में किसी फ्रेमवर्क/लाइब्रेरी की आवश्यकता नहीं है - नेटिव crypto पैकेज काम करते हैं।

FastComments SSO के साथ आपको कोई नए API endpoints लिखने की आवश्यकता नहीं है। बस अपनी secret key का उपयोग करके उपयोगकर्ता की जानकारी एन्क्रिप्ट करें और payload को comment widget को पास करें।

#### अपनी API Secret Key प्राप्त करें

आपकी API Secret <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">इस पेज</a> से प्राप्त की जा सकती है। आप यह पेज My Account में जाकर, API/SSO टाइल पर क्लिक करके, और फिर "Get API Secret Key" पर क्लिक करके भी ढूंढ सकते हैं।

#### Comment Widget पैरामीटर

Comment widget के लिए हाई-लेवल API दस्तावेज़ीकरण <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">यहाँ</a> मिल सकता है।

आइए इन पैरामीटर्स के अर्थ को और विस्तार से समझें।

Comment widget एक configuration object लेता है - यदि आप FastComments का उपयोग करके अपनी customer id (जिसे tenantId कहा जाता है) पास कर रहे हैं, तो आप यह पहले से ही पास कर रहे हैं।

SSO सक्षम करने के लिए, एक नया "sso" object पास करें, जिसमें निम्नलिखित पैरामीटर होने चाहिए। ये मान server side पर जेनरेट किए जाने चाहिए।

- userDataJSONBase64: JSON फॉर्मेट में उपयोगकर्ता का डेटा, जिसे फिर Base64 एन्कोड किया जाता है।
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64 से बनाया गया HMAC-SHA256 हैश।
- timestamp: Epoch timestamp, **milliseconds** में। यह भविष्य में नहीं, और दो दिन से अधिक पुराना नहीं होना चाहिए।
- loginURL: एक URL जिसे comment widget उपयोगकर्ता को लॉग इन कराने के लिए दिखा सकता है।
- logoutURL: एक URL जिसे comment widget उपयोगकर्ता को लॉग आउट कराने के लिए दिखा सकता है।
- loginCallback: login URL के स्थान पर प्रदान किए जाने पर, एक फ़ंक्शन जिसे comment widget login बटन पर क्लिक करने पर कॉल करेगा।
- logoutCallback: logout URL के स्थान पर प्रदान किए जाने पर, एक फ़ंक्शन जिसे comment widget logout बटन पर क्लिक करने पर कॉल करेगा।

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO क्लाइंट कोड'; isFunctional = false; code-example-end]

#### User Object

User object में निम्नलिखित schema होता है:
[inline-code-attrs-start title = 'User Object'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Required. 1k Characters Max. **/
    id: string;
    /** Required. 1k Characters Max. Note: Must be unique. **/
    email: string;
    /** Required. 1k Characters Max. Note: The username cannot be an email. Does not have to be unique. **/
    username: string;
    /** Optional. 3k Characters Max for URLs. Default is from gravatar based on email. Supports 64 encoded images, in which case the limit is 50k characters. **/ 
    avatar?: string;
    /** Optional. Default false. **/
    optedInNotifications?: boolean;
    /** Optional. Default false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optional. 100 Characters Max. This label will be shown next to their name. Default is Administrator/Moderator when applicable. **/
    displayLabel?: string;
    /** Optional. 500 Characters Max. This will be shown instead of the username. **/
    displayName?: string;
    /** Optional. 2k Characters Max. The user's name will link to this. **/
    websiteUrl?: string;
    /** Optional. Up to 100 groups per user. A group id may not be longer than 50 characters. **/
    groupIds?: string[];
    /** Optional. Denotes the user as an administrator. **/
    isAdmin?: boolean;
    /** Optional. Denotes the user as a moderator. **/
    isModerator?: boolean;
    /** Optional, default true. Set to false to enable the "activity" tab in the user's profile. **/
    isProfileActivityPrivate?: boolean;
    /** Optional, default false. Set to true to disable profile comments. **/
    isProfileCommentsPrivate?: boolean;
    /** Optional, default false. Set to true to disable direct messaging this user. **/
    isProfileDMDisabled?: boolean;
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of global badge IDs to assign. Limited to 30 badges. Order is respected. **/
        badgeIds: string[];
        /** Array of badge IDs scoped to the current page (urlId). Only displayed on the assigned page. **/
        pageBadgeIds?: string[];
        /** If true, replaces existing displayed badges. Global and page-scoped are overridden independently. **/
        override?: boolean;
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean;
    };
}
[inline-code-end]

#### मॉडरेटर और एडमिनिस्ट्रेटर

एडमिन और मॉडरेटर के लिए, `SSOUser` object में संबंधित `isAdmin` या `isModerator` फ्लैग पास करें।

#### नोटिफिकेशन

नोटिफिकेशन सक्षम या अक्षम करने के लिए, `optedInNotifications` का मान क्रमशः `true` या `false` सेट करें। जब उपयोगकर्ता पहली बार SSO payload में यह मान लेकर पेज लोड करता है, तो उनकी नोटिफिकेशन सेटिंग्स अपडेट हो जाएंगी।

इसके अतिरिक्त, यदि आप चाहते हैं कि उपयोगकर्ता उन पेजों की गतिविधि के लिए नोटिफिकेशन ईमेल प्राप्त करें जिनकी उन्होंने सदस्यता ली है (केवल इन-ऐप नोटिफिकेशन के बजाय), तो `optedInSubscriptionNotifications` को `true` सेट करें।

#### VIP उपयोगकर्ता और विशेष लेबल

आप वैकल्पिक "displayLabel" फ़ील्ड का उपयोग करके उपयोगकर्ता के नाम के बगल में एक विशेष लेबल प्रदर्शित कर सकते हैं।

#### अप्रमाणित उपयोगकर्ता

एक अप्रमाणित उपयोगकर्ता को दर्शाने के लिए, बस userDataJSONBase64, verificationHash, या timestamp को पॉपुलेट न करें। एक loginURL प्रदान करें।

ये उपयोगकर्ता टिप्पणी नहीं कर पाएंगे, और इसके बजाय उन्हें एक लॉगिन संदेश दिखाया जाएगा (संदेश, लिंक, या बटन, configuration के आधार पर)।

#### उपयोगकर्ता डेटा को सीरियलाइज़ और हैश करने के सीधे उदाहरण

उदाहरण के रूप में अधिक विवरण <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">यहाँ</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">यहाँ</a> (java) और <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">यहाँ</a> (php) उपलब्ध हैं।

हम समझते हैं कि कोई भी इंटीग्रेशन एक जटिल और कठिन प्रक्रिया हो सकती है। अपने प्रतिनिधि से संपर्क करने या <a href="https://fastcomments.com/auth/my-account/help" target="_blank">सपोर्ट पेज</a> का उपयोग करने में संकोच न करें।