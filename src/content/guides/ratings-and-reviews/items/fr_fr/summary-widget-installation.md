Voici le code Vanilla JS pour installer le widget Summary. La bibliothèque React dispose également de ce widget.

[inline-code-attrs-start title = 'Installation du widget Summary'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Le widget trouvera automatiquement les questions à afficher dans le résumé en fonction de la configuration du widget correspondante pour cette page/site.

### Filtrage des commentaires par note

Lorsque le widget de commentaires de la même page est également présent sur la page, et qu'il est configuré pour afficher la réponse de chaque commentateur sur son commentaire, les lignes de la répartition
deviennent cliquables. En cliquant sur une ligne, par exemple « 5 », le widget de commentaires est filtré pour n'afficher que les commentaires ayant donné cette réponse. En cliquant à nouveau sur la ligne, ou sur le X de la puce « Showing only »
dans le widget de commentaires, tous les commentaires sont à nouveau affichés. Cela ne nécessite aucune configuration : le widget Summary trouve le widget de commentaires avec le même `tenantId` et `urlId`.

Cela fonctionne pour les questions de type étoiles, pouces, emoji, échelle et choix unique. La répartition affichée par le widget de commentaires au-dessus de la zone de commentaire peut être cliquée de la même manière.

Si vous avez besoin du widget dans l'une de nos autres bibliothèques qui ne le possède pas, soumettez un ticket de support afin que nous sachions qu'il faut l'ajouter.