**ID du modèle :** `tos_enforcer`

Le modèle Modérateur est le point de départ recommandé si votre objectif est de réduire la charge de modération manuelle. Il examine les nouveaux commentaires et ceux signalés, et applique vos règles communautaires.

### Invite initiale intégrée

[inline-code-attrs-start title = 'Invite initiale du modèle Modérateur'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Vous voudrez presque toujours **augmenter cette invite** avec des exemples concrets de ce que votre site autorise ou non. La politique d'escalade propre à la plateforme (avertir avant de bannir, rechercher dans la mémoire avant de bannir) est déjà intégrée dans l'invite système que l'agent reçoit, vous n'avez donc pas besoin de la répéter.

### Déclencheurs

- **Nouveau commentaire publié** (`COMMENT_ADD`) - l'agent examine chaque nouveau commentaire.
- **Commentaire atteint un seuil de signalement** (`COMMENT_FLAG_THRESHOLD`, seuil par défaut : 3) - l'agent réévalue un commentaire qu'autres utilisateurs ont signalé.

### Outils autorisés

- [`mark_comment_approved`](#tools-overview) - utile pour les locataires en pré-modération où l'agent publie les commentaires approuvés et masque les autres.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Il ne peut pas publier de commentaires, voter, épingler, verrouiller, décerner des badges ou envoyer des courriels - l'invite est volontairement limitée.

### Ajouts recommandés avant la mise en ligne

- **Définissez les [Directives communautaires](#community-guidelines).** Quelques phrases de politique écrite suffisent; l'agent les applique à chaque exécution.
- **Placez `ban_user` derrière une [approbation](#approval-workflow).** Cela est activé par défaut dans la région UE (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)) et est recommandé partout.
- **Envisagez également de placer `mark_comment_spam` derrière une approbation** si vous avez du contenu à faible volume mais à enjeux élevés.
- **Placez `mark_comment_approved` derrière une approbation si vous pratiquez la pré-modération.** Approuver un mauvais commentaire le met devant les lecteurs ; verrouillez cette action jusqu'à ce que l'agent ait gagné votre confiance via [dry-run](#dry-run-mode).
- **Cochez « Inclure le facteur de confiance du commentateur, l'âge du compte, l'historique des bannissements et les commentaires récents »** dans les [Options de contexte](#context-options). Le modèle avertira beaucoup moins agressivement lorsqu'il pourra voir qu'une personne est un utilisateur de bonne foi depuis longtemps.

### Période d'essai recommandée

Exécutez ce modèle en [dry-run](#dry-run-mode) pendant au moins une semaine sur votre trafic réel avant de passer à Activé. Utilisez [Test Runs (Replays)](#test-runs-replays) pour également prévisualiser les 30 jours précédents.

---