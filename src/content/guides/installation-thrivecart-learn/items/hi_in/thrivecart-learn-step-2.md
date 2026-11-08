चरण 2 के लिए हमें अपना कोड स्निपेट कॉपी करना होगा। जांचें कि लाइन 50 पर "demo" नहीं लिखा है - आप यह सुनिश्चित करना चाहेंगे कि इसमें आपकी tenant id हो। यह आपके लिए पहले से भरा हुआ होना चाहिए।

अब आइए अपना ThriveCart-Learn-विशिष्ट FastComments कोड स्निपेट कॉपी करें।

यह काफी बड़ा है, क्योंकि ThriveCart के साथ इंटीग्रेशन में बहुत सारी सुविधाएं हैं, इसलिए बस कोड स्निपेट के ऊपरी दाएं कोने में Copy बटन पर क्लिक करें:

[inline-code-attrs-start title = 'ThriveCart Learn+ Comments Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let attemptsRemaining = 10;

        function tryLoad() {
            const simpleSSO = {optedInNotifications: true, optedInSubscriptionNotifications: true};
            let isAuthenticated = false;
            let profileLink = document.querySelector('.thrivecart-courses-header-profile-link');
            if (!profileLink) {
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // class is different for preview.
            }
            // broad email input field selector incase ThriveCart changes id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // allow preview to work - no email available.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }
            if (profileLink) {
                // use raw "img" query incase ThriveCart changes image class selector.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // use innerText incase ThriveCart changes how profile name is displayed.
                if (profileLink.innerText) {
                    isAuthenticated = true;
                    simpleSSO.username = profileLink.innerText;
                } else {
                    const bold = profileLink.querySelector('b');
                    if (bold && bold.innerText) {
                        isAuthenticated = true;
                        simpleSSO.username = bold.innerText;
                    }
                }
            } else {
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (user name/avatar). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user profile info found - waiting and trying again.');
                attemptsRemaining--;
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // sometimes TC uses multiple links the same page, so let's de-dupe them.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // trim marketing parameters and domain name
                url = window.location.pathname;
            }

            if (url) {
                url = url.replace('/starte-hier', '');
                url = url.replace('/start-here', '');
            }

            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: 'demo',
                urlId: url,
                simpleSSO: isAuthenticated ? simpleSSO : null
            });
        }

        tryLoad();

        function getPathnameFromUrl(url) {
            try {
                const parsedUrl = new URL(url);
                // trim marketing parameters and domain name
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // default to current, so at least it works sometimes
            }
        }

    })();
</script>
[inline-code-end]

अब इसे ThriveCart एडिटर में बाएं कोड ब्लॉक में पेस्ट करें। यह इस तरह दिखना चाहिए:

<div class="screenshot white-bg">
    <div class="title">कोड जोड़ा गया</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="कोड जोड़ा गया" />
</div>

बस! अब हमें सिर्फ प्रकाशित (publish) करना है:

<div class="screenshot white-bg">
    <div class="title">प्रकाशित करें</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="प्रकाशित करें" />
</div>

बस! अब जब आप प्रीव्यू करेंगे तो अपने कोर्स पर कमेंट बॉक्स देख सकेंगे, और वास्तविक उपयोगकर्ता **बिना साइन-इन किए या दूसरी बार अपना यूज़रनेम/ईमेल छोड़े** कमेंट कर पाएंगे।

### परीक्षण संबंधी नोट!

यदि आपने गुमनाम (anonymous) कमेंटिंग अक्षम की है, जो कि डिफ़ॉल्ट रूप से होती है, तो आप `Preview` मोड में `John Smith` उपयोगकर्ता के रूप में कमेंट नहीं कर पाएंगे। आपको एक प्रमाणीकरण त्रुटि मिलेगी क्योंकि डिफ़ॉल्ट `John Smith` उपयोगकर्ता के पास कोई ईमेल नहीं है। यदि आप परीक्षण करना चाहते हैं, तो हम सुझाव देते हैं कि आप एक कूपन कोड का उपयोग करें और अपनी साइट को एक वास्तविक उपयोगकर्ता की तरह देखें।