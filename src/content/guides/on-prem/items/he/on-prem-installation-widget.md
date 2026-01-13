קטעי הקוד וספריות של צד הלקוח עבור On-Prem זהים לאלו של מוצר ה-SaaS. עם זאת, עליך לציין את `apiHost` ואת נתיב הסקריפט הנכון:

[inline-code-attrs-start title = 'קוד תגובות עבור On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... ניתן גם להעביר מטען SSO וכו'.
    });
</script>
[inline-code-end]

הדוגמה שלמעלה היא דוגמה פשוטה מאוד. ניתן גם להשתמש בספריות של צד ראשון כגון React, Angular, Vue, Svelte וכו'.