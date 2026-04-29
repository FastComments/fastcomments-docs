Les courriels d’alerte de budget sont envoyés lorsque les dépenses d’un agent dépassent un pourcentage configurable de son plafond. Ils sont adressés aux personnes responsables de la facture.

### Comment fonctionnent les alertes

Chaque agent a un champ **Alert thresholds** dans le formulaire d’édition. Par défaut, il s’agit de `80%` et `100%`. Vous pouvez cocher ou décocher des seuils individuels, et ajouter d’autres pourcentages.

Quand les dépenses de l’agent dans une portée donnée (quotidienne ou mensuelle) franchissent un seuil pour la première fois pendant cette période, la plateforme envoie un courriel par destinataire. Le franchissement du même seuil de nouveau au cours de la même période (par exemple, si les dépenses sont retombées sous `80%` puis ont remonté) ne réenvoie **pas** le courriel.

Ceci est calculé par période : une nouvelle réinitialisation quotidienne relance la logique de détection des franchissements pour cette journée.

### Alertes à l'échelle du locataire

Le locataire (compte) a ses propres plafonds quotidiens et mensuels. Les alertes à l’échelle du locataire se déclenchent à des seuils fixes (`80%` et `100%`). Ceux-ci ne sont pas configurables par agent car ils s’appliquent à l’ensemble du locataire.

### Destinataires

Les alertes de budget sont envoyées à :

- Tous les utilisateurs marqués **Super admin** sur le locataire.
- Tous les utilisateurs marqués **Billing Admin** sur le locataire.

Cela inclut l’union des deux rôles : un utilisateur ayant les deux rôles reçoit un seul courriel.

### Pourquoi les deux rôles

Les Super admins sont généralement les opérateurs qui ont besoin de savoir qu’un agent atteint son plafond. Les Billing admins sont responsables de la facture et doivent connaître les pics de coûts, qu’ils gèrent ou non les agents au quotidien. Pour pouvoir réellement modifier l’agent (augmenter le plafond, le mettre en pause), le destinataire a également besoin du rôle **Customization Admin** — qui donne accès à la page d’édition de l’agent.

### Désinscription par utilisateur

Les destinataires qui se sont désinscrits des notifications administratives dans leur profil sont ignorés. Il s’agit du même interrupteur de désinscription qui contrôle les autres notifications administratives.

Si **tous** les destinataires sont désinscrits, l’alerte est consignée (niveau warning) et aucun courriel n’est envoyé.

### Contenu du courriel

Le courriel contient :

- Le **nom d’affichage de l’agent** et le nom interne.
- La **portée** qui a été franchie (par ex., « agent daily budget », « agent monthly budget », « account daily budget », « account monthly budget »).
- Le **pourcentage de seuil** franchi.
- **L’utilisation** dans la devise du locataire.
- Le **plafond** dans la devise du locataire.
- Un **lien de connexion signé en un clic** qui mène le destinataire directement vers :
  - La page d’édition de l’agent, pour les alertes au niveau de l’agent.
  - La page de la liste AI Agents, pour les alertes à l’échelle du locataire.

Le lien est pré-authentifié, ainsi le destinataire est à un clic d’augmenter le plafond ou de désactiver l’agent.

### Comment les seuils se déclenchent

La plateforme suit les seuils déjà déclenchés pendant la période, séparément pour l’agent et pour le locataire. Ainsi :

- Le franchissement de `80%` puis de `100%` pendant la même période déclenche les deux, dans cet ordre.
- Passer directement de `0%` à `100%` en une seule augmentation déclenche le seuil **le plus élevé** franchi (`100%`), et non `80%`, de sorte que l’alerte la plus sévère est celle qui est envoyée.

### Quand vous cessez de recevoir des alertes

Si les dépenses de l’agent n’atteignent jamais le seuil suivant pendant cette période, vous ne recevrez pas d’autres courriels au cours de cette période. La prochaine réinitialisation quotidienne (ou mensuelle) efface le suivi.

### Désactiver les alertes

Décochez le seuil que vous ne souhaitez pas recevoir. Si vous ne voulez aucune alerte pour un agent spécifique, décochez tous les pourcentages. Les alertes à l’échelle du locataire ne peuvent pas être désactivées par agent (elles s’appliquent à l’ensemble du locataire).

### Voir aussi

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - ce qui se passe lorsque le plafond est complètement atteint.
- [Cost Model](#cost-model) - ce qui est mesuré.