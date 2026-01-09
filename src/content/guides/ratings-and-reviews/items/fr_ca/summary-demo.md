Voici une petite démo du widget de résumé :

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // supprimer les paramètres de requête
    });
</script>

### Mise en cache

Notez que les résumés sont mis en cache pendant 30 secondes, ou cinq minutes s'il y a un grand nombre d'avis. En conséquence, votre avis peut ne pas apparaître immédiatement dans le résumé, mais cela nous permet de réduire le coût de diffusion du widget de résumé.

---