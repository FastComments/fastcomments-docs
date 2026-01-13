עכשיו אנו הולכים להעתיק את הקוד שלנו. אם בקטע הקוד כתוב `tenantId: "demo"` בשורה 6 אז עליך להיכנס לחשבון FastComments שלך
ואז לרענן את הדף כדי שקטע הקוד שהועתק יכיל את מזהה החשבון שלך.

[inline-code-attrs-start title = 'קטע של Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

כעת הדבק אותו בעורך ולחץ שמור:

<div class="screenshot white-bg">
    <div class="title">הוסף את קוד FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="הוסף את קוד FastComments" />
</div>

... אז שמור את האתר שלך. זה הכל!

---