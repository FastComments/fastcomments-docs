---
דוגמה זו משתמשת בקוד מותאם כדי להיות תואמת ל-Wix. **לא תוכלו להשתמש בקטעי הקוד של FastComments מההדרכות האחרות.**

Open the form to add our custom HTML dialog by clicking `Enter Code` and selecting `HTML`:

<div class="screenshot white-bg">
    <div class="title">שלב 3: פתח חלונית HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="שלב 3: פתח חלונית HTML" />
</div>

Copy the following HTML snippet and paste it into the dialog, and click Update:

[inline-code-attrs-start title = 'קטע קוד לתגובות Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">שלב 3: הדבק ושמור</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="שלב 3: הדבק ושמור" />
</div>

You should now see a very tiny version of the comment widget:

<div class="screenshot white-bg">
    <div class="title">שלב 3: התוצאה</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="שלב 3: התוצאה" />
</div>

Next we will position and size this to fit our page.

---