---
כעת נגלול למטה אל השורה `100`:

<div class="screenshot white-bg">
    <div class="title">גלול לשורה 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="גלול לשורה 100" />
</div>

כעת העתק את קטע הקוד הבא, שנוצר **במיוחד עבור Shopify - אל תשתמש בקטעי קוד ממדריכים אחרים**:

[inline-code-attrs-start title = 'קטע FastComments ל-Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        urlId: window.location.pathname
    });
</script>
[inline-code-end]

כעת נרצה להניח את הסמן על `line 101` - מיד אחרי ה-`</div>` - ולהדביק. אמור להיות לכם משהו כזה:

<div class="screenshot white-bg">
    <div class="title">הוסף את קוד FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="הוסף את קוד FastComments" />
</div>

כעת נוכל לשמור:

<div class="screenshot white-bg">
    <div class="title">שמור</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="שמור" />
</div>

---