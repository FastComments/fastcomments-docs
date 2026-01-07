אנחנו יכולים לחבר בקלות את FastComments עם Memberstack באמצעות קטע קוד קטן:

[inline-code-attrs-start title = 'FastComments Memberstack Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

כאשר המשתמש מבקר באתר או באפליקציה שלך בזמן שהוא מחובר דרך Memberstack, הוא יתחבר אוטומטית ל-FastComments והתגובות שלו
יסומנו כ-`Verified`.

**הם גם יוכלו להגיב על ידי השארת שמם ואימייל שלהם, אם הם לא מחוברים.**

### תגובות לחברים בלבד

אם תרצו **תגובות לחברים בלבד**, נסו את קטע הקוד הבא, אבל שנו את `https://example.com/login` למקום שאליו תרצו שהמשתמשים יגיעו כשהם לוחצים על כפתור `Login`:

[inline-code-attrs-start title = 'Exclusive FastComments Memberstack Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
