לא מומלץ להוסיף את FastComments דרך בונה העמודים של BigCommerce, מכיוון שבמקרה כזה יש להוסיף את הקוד ידנית לכל דף רצוי.

עם זאת, אם זה רצוי, יש להשתמש בקטע הקוד הבא. קטעי קוד ממדריכים אחרים לא יעבדו בשל אופי BigCommerce:

---
[inline-code-attrs-start title = 'קטע עבור בונה העמודים של BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

---