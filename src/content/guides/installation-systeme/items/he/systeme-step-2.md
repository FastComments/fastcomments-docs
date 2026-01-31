עכשיו נעתיק את הקוד שלנו. אם בקטע הקוד מופיע `tenantId: "demo"` בשורה 6 אז עליך להיכנס לחשבון FastComments שלך
ואז לרענן את הדף הזה כדי שקטע הקוד המועתק יכיל את מזהה החשבון שלך.

[inline-code-attrs-start title = 'קטע של Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

עכשיו הדבק אותו בעורך ולחץ על שמור:

<div class="screenshot white-bg">
    <div class="title">הוסף את קוד FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="הוסף את קוד FastComments" />
</div>

... אז שמור את האתר שלך. זהו!