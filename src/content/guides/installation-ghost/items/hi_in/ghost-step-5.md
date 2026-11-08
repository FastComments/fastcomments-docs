अब हमें यह पहचानना होगा कि FastComments.com विजेट कोड कहाँ जोड़ना है।

यदि आप डिफ़ॉल्ट `casper` थीम का उपयोग कर रहे हैं, तो आपको लाइन `82` पर इस तरह का एक सेक्शन दिखाई देगा:

<div class="screenshot white-bg">
    <div class="title">अक्षम कमेंट सेक्शन</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="अक्षम कमेंट सेक्शन" />
</div>

यदि आप अन्य थीम का उपयोग कर रहे हैं, तो आपको यह नहीं दिखाई देगा, और आपको इस कोड को अंतिम `</section>` के बाद जोड़ना होगा:

[inline-code-attrs-start title = 'सेक्शन उदाहरण'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

आपके पास कुछ इस तरह तैयार होना चाहिए:

<div class="screenshot white-bg">
    <div class="title">कमेंट कोड के लिए तैयार टेम्पलेट</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="कमेंट कोड के लिए तैयार टेम्पलेट" />
</div>

तैयार होने के बाद, FastComments.com विजेट कोड कॉपी करें:

[inline-code-attrs-start title = 'Ghost FastComments.com कमेंट कोड स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let simpleSSO = null;

        \{{#if access}}
            \{{#if @member}}
                simpleSSO = {
                    id: '\{{ @member.uuid }}',
                    email: '\{{@member.email}}',
                    username: '\{{@member.name}}',
                    avatar: '\{{ @member.avatar_image }}',
                    optedInNotifications: true,
                    optedInSubscriptionNotifications: true,
                    displayLabel: '\{{@member.labels}}'
                }
            \{{/if}}
        \{{/if}}

        window.fcConfigs = [{
            target: '#fastcomments-widget',
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        }];
    })();
</script>
[inline-code-end]

...और यह इस तरह दिखना चाहिए:

<div class="screenshot white-bg">
    <div class="title">FastComments.com कमेंट कोड जोड़ें</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="FastComments.com कमेंट कोड जोड़ें" />
</div>

कोडिंग पूरी हुई। अब हमें बस अपनी थीम को फिर से इम्पोर्ट करना है!