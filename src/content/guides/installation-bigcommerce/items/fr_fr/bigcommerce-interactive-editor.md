Il n'est pas recommandé d'ajouter FastComments via le Page Builder de BigCommerce, car le code doit ensuite être ajouté manuellement à chaque page souhaitée.

Cependant, si vous souhaitez le faire, le fragment de code suivant doit être utilisé. Les extraits de code provenant d'autres tutoriels ne fonctionneront pas en raison de la nature de BigCommerce :

[inline-code-attrs-start title = 'Extrait pour BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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