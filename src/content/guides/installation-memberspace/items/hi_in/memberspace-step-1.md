हम एक छोटे कोड स्निपेट के साथ FastComments को MemberSpace के साथ आसानी से जोड़ सकते हैं:

[inline-code-attrs-start title = 'FastComments MemberSpace स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo';
        const ALLOW_ANON = false;
        const LOGIN_URL = 'https://example.com/login';
        const PLAN_DISPLAY_LABELS = {
            'VIP Plan': 'VIP'
        };
        let lastInstance;
    
        function tick() {
            if (!window.MemberSpace) {
                return setTimeout(tick, 100);
            }
            if (!window.FastCommentsUI) {
                return setTimeout(tick, 100);
            }
            const target = document.getElementById('fastcomments-widget');
            if (!target) {
                return setTimeout(tick, 100);
            }
            const data = MemberSpace.getMemberInfo();
            if (data.isLoggedIn && data.memberInfo) {
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: {
                        displayLabel: getDisplayLabel(data.memberInfo),
                        username: data.memberInfo.name,
                        email: data.memberInfo.email,
                        avatar: data.memberInfo.profileImageUrl
                    }
                });
            } else if (lastInstance) {
                lastInstance.destroy();
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: getAnonSSOConfig()
                });
            }
        }
    
        function getAnonSSOConfig() {
            if (ALLOW_ANON) {
                return undefined;
            }
            return {
                loginURL: LOGIN_URL
            };
        }
        
        function getDisplayLabel(memberInfo) {
            if (!memberInfo.memberships) {
                return;
            }
            for (const membership of memberInfo.memberships) {
                const label = PLAN_DISPLAY_LABELS[membership.name];
                if (label) {
                    return label
                }
            }
        }
    
        tick();
    })();
</script>
[inline-code-end]

जब उपयोगकर्ता MemberStack के माध्यम से लॉग इन होकर आपकी साइट या एप्लिकेशन पर आता है, तो वह स्वतः ही FastComments में लॉग इन हो जाएगा और उसकी टिप्पणियाँ `Verified` के रूप में चिह्नित होंगी।

इसके अतिरिक्त, उपरोक्त उदाहरण में यदि आपके पास `VIP Plan` नामक कोई सब्सक्रिप्शन प्लान है, तो हम उपयोगकर्ता के नाम के आगे एक `VIP` बैज प्रदर्शित करेंगे। आप अधिक प्लान जोड़ने के लिए उदाहरण को संपादित कर सकते हैं। यदि आपके कोई प्रश्न हैं तो सपोर्ट से संपर्क करें।

### गुमनाम टिप्पणी (Anonymous Commenting) की अनुमति दें

यदि आप **गुमनाम टिप्पणी** भी चाहते हैं, तो ALLOW_ANON को इस तरह true पर सेट करें:

    const ALLOW_ANON = true;

साथ ही, `https://example.com/login` को उस पते में बदलना न भूलें जहाँ आप चाहते हैं कि उपयोगकर्ता `Login` बटन पर क्लिक करने पर जाएँ:

इस तरह, यदि उपयोगकर्ता आपकी मेंबर साइट में लॉग इन नहीं है, तो उनके पास टिप्पणी करने के लिए अपना नाम और ईमेल दर्ज करने का विकल्प होगा।