לאחר מכן עלינו לזהות היכן להוסיף את קוד הווידג'ט של FastComments.com.

אם אתה משתמש בערכת הנושא המוגדרת כברירת מחדל `casper`, תראה קטע כזה בשורה `82`:

<div class="screenshot white-bg">
    <div class="title">מדור תגובות מושבת</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="מדור תגובות מושבת" />
</div>

אם אתה משתמש בנושאים אחרים, לא תראה זאת, ותצטרך להוסיף קוד זה אחרי ה-`</section>` האחרון:

[inline-code-attrs-start title = 'דוגמה לסעיף'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

עליך שיהיה משהו כזה מוכן:

<div class="screenshot white-bg">
    <div class="title">התבנית מוכנה לקוד תגובות</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="התבנית מוכנה לקוד תגובות" />
</div>

לאחר שהכל מוכן, העתק את קוד הווידג'ט של FastComments.com:

[inline-code-attrs-start title = 'קטע קוד תגובות Ghost של FastComments.com'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...והוא אמור להיראות כך:

<div class="screenshot white-bg">
    <div class="title">הוסף את קוד התגובות של FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="הוסף את קוד התגובות של FastComments.com" />
</div>

הקידוד הושלם. עכשיו רק נשאר לייבא מחדש את ערכת הנושא שלנו!