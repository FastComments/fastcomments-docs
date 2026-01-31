Ne preporučuje se dodavanje FastComments-a putem BigCommerceovog Page Buildera jer tada kôd mora biti ručno dodan na svaku željenu stranicu.

Međutim, ako je to poželjno, mora se koristiti sljedeći isječak koda. Isječci koda iz drugih vodiča neće raditi zbog prirode BigCommercea:

[inline-code-attrs-start title = 'Isječak za BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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