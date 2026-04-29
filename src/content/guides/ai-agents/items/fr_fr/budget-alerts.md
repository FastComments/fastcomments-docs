---
Les e-mails d'alerte de budget sont envoyés lorsqu'une dépense d'agent dépasse un pourcentage configurable de son plafond. Ils sont adressés aux personnes qui paient la facture.

### Comment fonctionnent les alertes

Chaque agent possède un champ **Seuils d'alerte** dans le formulaire d'édition. Par défaut, il contient `80%` et `100%`. Vous pouvez cocher ou décocher des seuils individuels, et ajouter d'autres pourcentages.

Lorsque la dépense de l'agent dans une portée donnée (quotidienne ou mensuelle) franchit un seuil pour la première fois pendant cette période, la plateforme envoie un e-mail par destinataire. Le franchissement du seuil à nouveau plus tard dans la même période (par ex., la dépense est retombée en dessous de 80% puis est repassée au-dessus) **n'entraîne pas** un nouvel envoi.

Ceci se fait par période : une nouvelle réinitialisation quotidienne relance la logique de franchissement des seuils pour cette journée.

### Alertes au niveau du tenant (compte)

Le tenant (compte) a ses propres plafonds quotidiens et mensuels. Les alertes au niveau du tenant se déclenchent à des seuils fixes (`80%` et `100%`). Elles ne sont pas configurables par agent car elles s'appliquent à l'ensemble du tenant.

### Destinataires

Les alertes de budget sont envoyées à :

- Tout utilisateur marqué **Super administrateur** sur le tenant.
- Tout utilisateur marqué **Administrateur de facturation** sur le tenant.

Cela inclut l'union des deux rôles : un utilisateur ayant les deux rôles reçoit un seul e-mail.

### Pourquoi les deux rôles

Les super administrateurs sont généralement les opérateurs qui doivent savoir qu'un agent atteint son plafond. Les administrateurs de facturation possèdent la facture et doivent être informés des pics de coût, même s'ils ne gèrent pas les agents au quotidien. Pour pouvoir réellement modifier l'agent (augmenter le plafond, le mettre en pause), le destinataire a aussi besoin du rôle **Administrateur de personnalisation** – qui contrôle l'accès à la page d'édition de l'agent.

### Désinscription par utilisateur

Les destinataires qui se sont désinscrits des notifications d'administration dans leur profil sont ignorés. Il s'agit du même interrupteur de désinscription qui contrôle les autres notifications d'administration.

Si **tous** les destinataires sont désinscrits, l'alerte est journalisée (niveau d'avertissement) et aucun e-mail n'est envoyé.

### Contenu de l'e-mail

L'e-mail contient :

- Le **nom affiché de l'agent** et le nom interne.
- Le **périmètre** qui a été franchi (p. ex., "budget quotidien de l'agent", "budget mensuel de l'agent", "budget quotidien du compte", "budget mensuel du compte").
- Le **pourcentage du seuil** franchi.
- **Utilisation** dans la devise du tenant.
- **Plafond** dans la devise du tenant.
- Un **lien de connexion signé en un clic** qui amène le destinataire directement vers :
  - la page d'édition de l'agent, pour les alertes au niveau de l'agent.
  - la page de la liste des agents IA, pour les alertes au niveau du tenant.

Le lien est pré-authentifié, ainsi le destinataire est à un clic d'augmenter le plafond ou de désactiver l'agent.

### Comment les seuils se déclenchent

La plateforme suit quels seuils ont déjà été déclenchés pendant cette période, séparément pour l'agent et pour le tenant. Ainsi :

- Franchir 80% puis 100% dans la même période déclenche les deux, dans l'ordre.
- Passer directement de 0% à 100% en un seul saut déclenche le seuil **le plus élevé** franchi (100%), et non 80%, donc l'alerte la plus sévère est celle envoyée.

### Quand vous cessez de recevoir des alertes

Si la dépense de l'agent n'atteint jamais le seuil suivant pendant cette période, vous ne recevez pas d'autres e-mails pendant cette période. La prochaine réinitialisation quotidienne (ou mensuelle) efface le suivi.

### Désactivation des alertes

Décochez le seuil que vous ne souhaitez pas. Si vous ne voulez aucune alerte pour un agent spécifique, décochez tous les pourcentages. Les alertes au niveau du tenant ne peuvent pas être désactivées par agent (elles sont applicables à l'ensemble du tenant).

### Voir aussi

- [Aperçu des budgets](#budgets-overview).
- [Raisons de rejet](#drop-reasons) - ce qui se passe lorsque le plafond est atteint.
- [Modèle de coût](#cost-model) - ce qui est mesuré.

---