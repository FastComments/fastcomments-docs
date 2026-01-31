---
Ne priporoča se dodajanje FastComments prek Page Builderja BigCommerce, saj je treba nato kodo ročno dodati na vsako želeno stran.

Če pa je to zaželeno, je treba uporabiti naslednji del kode. Delčki kode iz drugih vadnic ne bodo delovali zaradi narave BigCommerce:

[inline-code-attrs-start title = 'Vstavek za Page Builder BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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