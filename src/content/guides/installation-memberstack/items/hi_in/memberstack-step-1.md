हम एक छोटे कोड स्निपेट के साथ FastComments को Memberstack के साथ आसानी से जोड़ सकते हैं:

[inline-code-attrs-start title = 'FastComments Memberstack स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.$memberstackDom.getCurrentMember().then(({data: member}) => {
        if (member) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    username: member.customFields.name || member.auth.email.replace(/@.+/, ''),
                    email: member.auth.email,
                    avatar: member.customFields.avatar
                }
            });
        } else {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: null
            });
        }
    })
</script>
[inline-code-end]

जब उपयोगकर्ता Memberstack के माध्यम से लॉग इन होकर आपकी साइट या एप्लिकेशन पर आता है, तो वह स्वचालित रूप से FastComments में लॉग इन हो जाएगा और उसकी टिप्पणियाँ
`Verified` के रूप में चिह्नित होंगी।

**यदि वे लॉग इन नहीं हैं, तो वे अपना नाम और ईमेल छोड़कर भी टिप्पणी कर सकेंगे।**

### केवल-सदस्य टिप्पणी

यदि आप **केवल-सदस्य टिप्पणी (member-only commenting)** चाहते हैं, तो निम्नलिखित कोड स्निपेट आज़माएँ, लेकिन `https://example.com/login` को उस स्थान से बदलें जहाँ आप चाहते हैं कि उपयोगकर्ता `Login` बटन पर क्लिक करने पर जाएँ:

[inline-code-attrs-start title = 'एक्सक्लूसिव FastComments Memberstack स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.$memberstackDom.getCurrentMember().then(({data: member}) => {
        if (member) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    username: member.customFields.name || member.auth.email.replace(/@.+/, ''),
                    email: member.auth.email,
                    avatar: member.customFields.avatar
                }
            });
        } else {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    loginURL: 'https://example.com/login'
                }
            });
        }
    })
</script>
[inline-code-end]