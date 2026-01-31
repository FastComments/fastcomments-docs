Nije preporučljivo dodavati FastComments putem BigCommerce-ovog Page Builder-a, jer se tada kod mora ručno dodavati na svaku željenu stranicu.

Međutim, ako je to željeno, mora se koristiti sljedeći isječak koda. Isječci koda iz drugih tutorijala neće raditi zbog specifičnosti BigCommerce-a:

[inline-code-attrs-start title = 'BigCommerce Page Builder isječak koda'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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