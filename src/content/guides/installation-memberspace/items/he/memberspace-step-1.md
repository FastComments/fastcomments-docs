אנחנו יכולים לחבר בקלות את FastComments עם MemberSpace באמצעות קטע קוד קטן:

[inline-code-attrs-start title = 'FastComments MemberSpace Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

כאשר המשתמש מבקר באתר או באפליקציה שלך בזמן שהוא מחובר דרך MemberStack, הוא יתחבר אוטומטית ל-FastComments והתגובות שלו
יסומנו כ-`Verified`.

בנוסף, בדוגמה לעיל, אם יש לך תוכנית מנוי בשם `VIP Plan`, נציג תג `VIP` ליד שם המשתמש. אתה יכול לערוך את הדוגמה כדי
להוסיף תוכניות נוספות. פנה לתמיכה אם יש לך שאלות.

### אפשר תגובות אנונימיות

אם תרצה גם **תגובות אנונימיות**, הגדר את ALLOW_ANON ל-true כך:

    const ALLOW_ANON = true;

כמו כן, זכור לשנות את `https://example.com/login` למקום שאליו אתה רוצה שהמשתמשים ילכו כשהם לוחצים על כפתור `Login`:

בדרך זו למשתמשים תהיה האפשרות להזין את שמם ואימייל שלהם כדי להגיב אם הם לא מחוברים לאתר החברים שלך.
