---
Ci-dessous se trouve le code Vanilla JS pour installer le widget de résumé. La bibliothèque React contient également ce widget.

[inline-code-attrs-start title = 'Installation du widget de résumé'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Le widget trouvera automatiquement les questions à afficher dans le résumé en fonction de la configuration correspondante du widget pour cette page/site.

Si vous avez besoin du widget dans l'une de nos autres bibliothèques qui ne l'inclut pas, ouvrez un ticket de support afin que nous sachions l'ajouter.

---