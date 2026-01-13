[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Par défaut, les nouveaux commentaires en direct apparaissent en haut de la liste des commentaires au fur et à mesure qu'ils sont publiés en temps réel.

Lorsque cette option est activée, les nouveaux commentaires en direct sont ajoutés en bas de la liste. Cela affecte la façon dont les commentaires s'affichent lorsqu'ils sont publiés en direct pendant que les utilisateurs consultent le fil de discussion.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Avec ce paramètre activé :
- Les nouveaux commentaires en direct publiés par d'autres utilisateurs apparaîtront en bas de la liste des commentaires
- Les utilisateurs verront les nouveaux commentaires s'afficher sous les commentaires existants en temps réel
- Cela n'affecte que les mises à jour des commentaires en direct - pas le chargement initial de la page
- Cela peut aider à maintenir la fluidité de la lecture lorsque les utilisateurs suivent une discussion

Notez que ce paramètre n'affecte que l'emplacement des nouveaux commentaires en direct au moment de leur arrivée en temps réel. Il n'affecte pas l'ordre de tri initial lors du chargement de la page.