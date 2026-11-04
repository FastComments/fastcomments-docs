דוגמה זו משתמשת בקוד מותאם אישית לצורך תאימות ל-Wix. **לא תוכלו להשתמש בקטעי הקוד של FastComments ממדריכים אחרים.**

פתחו את הטופס כדי להוסיף את דיאלוג ה-HTML המותאם אישית שלנו על ידי לחיצה על `Enter Code` ובחירה ב-`HTML`:

<div class="screenshot white-bg">
    <div class="title">שלב 3: פתיחת דיאלוג ה-HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="שלב 3: פתיחת דיאלוג ה-HTML" />
</div>

העתיקו את קטע ה-HTML הבא והדביקו אותו בדיאלוג, ולאחר מכן לחצו על Update:

[inline-code-attrs-start title = 'קטע קוד תגובות של Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">שלב 3: הדבקה ושמירה</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="שלב 3: הדבקה ושמירה" />
</div>

כעת אמורה להופיע בפניכם גרסה קטנטנה של ווידג'ט התגובות:

<div class="screenshot white-bg">
    <div class="title">שלב 3: התוצאה</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="שלב 3: התוצאה" />
</div>

בשלב הבא נמקם ונגדיר את הגודל שלו כך שיתאים לעמוד שלנו.