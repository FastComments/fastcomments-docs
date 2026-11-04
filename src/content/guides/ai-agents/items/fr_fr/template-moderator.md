**ID du modèle :** `tos_enforcer`

Le modèle Moderator est le point de départ recommandé si votre objectif est de réduire la charge de modération manuelle. Il examine les nouveaux commentaires et ceux signalés, et applique les règles de votre communauté.

Vous voudrez presque toujours **enrichir le prompt intégré** avec des exemples concrets de ce que votre site autorise et interdit. La politique d'escalade de la plateforme (avertir avant de bannir, consulter la mémoire avant de bannir) est déjà intégrée au prompt système que l'agent reçoit, vous n'avez donc pas besoin de la répéter.

### Déclencheurs

- **Nouveau commentaire publié** (`COMMENT_ADD`) - l'agent examine chaque nouveau commentaire.
- **Un commentaire franchit un seuil de signalement** (`COMMENT_FLAG_THRESHOLD`, seuil par défaut : 3) - l'agent réévalue un commentaire que d'autres utilisateurs ont signalé.

### Outils autorisés

- [`mark_comment_approved`](#tools-overview) - utile pour les tenants en pré-modération où l'agent publie les commentaires propres et masque les autres.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Il ne peut pas publier de commentaires, voter, épingler, verrouiller, attribuer des badges ni envoyer d'e-mails - le prompt est volontairement restreint.

### Ajouts recommandés avant la mise en production

- **Définissez des [règles communautaires](#community-guidelines).** Quelques phrases de politique écrite suffisent ; l'agent les applique à chaque exécution.
- **Soumettez `ban_user` à une [approbation](#approval-workflow).** C'est activé par défaut dans la région UE (voir [Conformité au DSA de l'UE, article 17](#eu-dsa-compliance)) et recommandé partout ailleurs.
- **Envisagez également de soumettre `mark_comment_spam` à une approbation** si vous avez un contenu à faible volume mais à enjeux élevés.
- **Soumettez `mark_comment_approved` à une approbation si vous utilisez la pré-modération.** Approuver un mauvais commentaire le place devant les lecteurs ; soumettez-le à approbation jusqu'à ce que l'agent ait gagné la confiance grâce au mode dry-run.
- **Cochez « Inclure le facteur de confiance du commentateur, l'âge du compte, l'historique de bannissement et les commentaires récents »** dans les [options de contexte](#context-options). Le modèle avertira beaucoup moins agressivement lorsqu'il peut voir que quelqu'un est un utilisateur de longue date et de bonne foi.

### Fenêtre de dry-run recommandée

Exécutez ce modèle en [dry-run](#dry-run-mode) pendant au moins une semaine sur votre trafic réel avant de passer à Activé. Utilisez les [exécutions de test (replays)](#test-runs-replays) pour également prévisualiser sur les 30 derniers jours.