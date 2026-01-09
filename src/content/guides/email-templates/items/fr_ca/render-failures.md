Puisque les modèles d'e-mail prennent en charge des variables et de la logique, il est possible de créer des modèles
qui échouent à être rendus, ou parfois échouent à être rendus.

Cela peut être très frustrant à diagnostiquer et à déboguer, surtout si c'est un problème intermittent, ou
si cela n'arrive que lorsque les données ont une certaine apparence.

Pour aider, FastComments Email Templates offre quelques fonctionnalités :

1. Si le modèle ne parvient pas à être prévisualisé, il ne peut pas être enregistré. Un message d'erreur sera affiché.
2. Les échecs de rendu des modèles sont suivis et signalés dans l'interface d'administration.

Le deuxième point décrit des échecs de rendu qui surviennent en production. Autrement dit, vous créez un modèle qui se prévisualise
correctement - mais qui échoue plus tard pour une raison quelconque. Par exemple, si nous avons ceci dans notre modèle :

    <% if (comment.commenterEmail.includes('test') { %>

Cela peut parfois échouer si les commentaires anonymes sont activés, puisque l'adresse e-mail ne sera pas toujours
disponible. Alors, comment le découvrir ?

La réponse est que les erreurs sont signalées à deux endroits. Tout d'abord, la liste des modèles elle-même
affiche un compteur d'erreurs de rendu pour chaque modèle.

Ensuite, en affichant un modèle, nous pouvons voir, pour chaque erreur, le nombre de fois que le modèle
n'a pas pu être rendu.

Un bouton de réinitialisation se trouve à côté de chaque erreur et de son compteur, afin que nous puissions remettre le compteur
à zéro après avoir résolu le problème.