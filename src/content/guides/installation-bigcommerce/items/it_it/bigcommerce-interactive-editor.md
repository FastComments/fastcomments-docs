Non è consigliato aggiungere FastComments tramite il Page Builder di BigCommerce, poiché in tal caso il codice deve essere aggiunto manualmente a ogni singola pagina desiderata.

Tuttavia, se questo è desiderato, deve essere utilizzato il seguente snippet di codice. Gli snippet di altri tutorial non funzioneranno a causa della natura di BigCommerce:

[inline-code-attrs-start title = 'Snippet per il Page Builder di BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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