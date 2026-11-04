עבור שלב 2 עלינו להעתיק את קטע הקוד שלנו. ודאו ששורה 50 אינה אומרת "demo" - תרצו לוודא ששם מופיע מזהה הדייר (tenant id) שלכם. הוא אמור להיות מאוכלס עבורכם אוטומטית.

כעת נעתיק את קטע הקוד של FastComments הייחודי ל-ThriveCart-Learn.

הוא די גדול, מכיוון שהאינטגרציה עם ThriveCart כוללת הרבה תכונות, ולכן פשוט לחצו על כפתור ההעתקה בפינה השמאלית העליונה של קטע הקוד:

[inline-code-attrs-start title = 'קוד תגובות עבור ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // המחלקה שונה בתצוגה מקדימה.
            }
            // סלקטור רחב לשדה קלט האימייל למקרה ש-ThriveCart משנה את ה-id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // מאפשר לתצוגה המקדימה לעבוד - אין אימייל זמין.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // מגדילים זמן המתנה אחרי 5 ניסיונות למקרה של אינטרנט איטי.
            }
            if (profileLink) {
                // משתמשים בשאילתת "img" גולמית למקרה ש-ThriveCart משנה את סלקטור המחלקה של התמונה.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // משתמשים ב-innerText למקרה ש-ThriveCart משנה את אופן הצגת שם הפרופיל.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // מגדילים זמן המתנה אחרי 5 ניסיונות למקרה של אינטרנט איטי.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // לפעמים TC משתמש במספר קישורים לאותו דף, ולכן נסיר כפילויות.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // מסירים פרמטרים שיווקיים ואת שם הדומיין
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
                // מסירים פרמטרים שיווקיים ואת שם הדומיין
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // ברירת מחדל לנוכחי, כך שלפחות זה עובד לפעמים
            }
        }

    })();
</script>
[inline-code-end]

כעת הדביקו אותו לבלוק הקוד משמאל בעורך של ThriveCart. זה אמור להיראות כך:

<div class="screenshot white-bg">
    <div class="title">הקוד נוסף</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="הקוד נוסף" />
</div>

זהו זה! כעת רק עלינו לפרסם:

<div class="screenshot white-bg">
    <div class="title">פרסום</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="פרסום" />
</div>

זהו זה! כעת אמורה להופיע תיבת התגובות בקורס שלכם בתצוגה המקדימה, ומשתמשים אמיתיים יוכלו להשאיר תגובות **מבלי להתחבר או להזין שוב את שם המשתמש/האימייל שלהם**.

### הערה לבדיקה!

אם הערות אנונימיות מושבתות אצלכם - מה שהוא ברירת המחדל - לא תוכלו להשאיר תגובות במצב `Preview` כמשתמש `John Smith`. תקבלו שגיאת אימות, מכיוון שלמשתמש ברירת המחדל `John Smith` אין אימייל. אם ברצונכם לבדוק, אנו ממליצים להשתמש בקוד קופון ולעבור על האתר שלכם כמו משתמש אמיתי.