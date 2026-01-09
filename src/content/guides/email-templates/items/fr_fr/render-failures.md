---
Puisque les modèles d'e-mail prennent en charge des variables et de la logique, il est possible de créer des modèles qui ne s'affichent pas, parfois de manière intermittente.

Cela peut être très frustrant à diagnostiquer et à déboguer, surtout si le problème est intermittent ou s'il ne survient que lorsque les données ont un certain aspect.

Pour aider, FastComments Email Templates propose quelques fonctionnalités :

1. Si le modèle échoue lors de la prévisualisation, il ne peut pas être enregistré. Un message d'erreur sera affiché.
2. Les échecs de rendu des modèles sont suivis et rapportés dans l'interface d'administration.

Le deuxième point décrit les échecs de rendu qui se produisent en production. Autrement dit, vous créez un modèle qui se prévisualise correctement - mais qui échoue plus tard pour une raison quelconque. Par exemple, si nous avons ceci dans notre modèle :

    <% if (comment.commenterEmail.includes('test') { %>

Cela peut parfois échouer si nous avons l'option de commentaire anonyme activée, car l'e-mail ne sera pas toujours disponible. Alors comment le découvrir ?

La réponse est que les erreurs sont affichées à deux endroits. D'abord, la liste des modèles elle-même montre un nombre d'erreurs de rendu pour chaque modèle.

Ensuite, en consultant un modèle, nous pouvons voir un décompte, par type d'erreur, du nombre de fois où le modèle a échoué à se rendre.

Un bouton de réinitialisation est situé à côté de chaque erreur et de son compteur, afin que nous puissions réinitialiser le compteur après avoir résolu un problème.

---