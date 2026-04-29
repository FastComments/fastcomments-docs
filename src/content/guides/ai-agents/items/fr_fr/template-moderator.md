**Template ID:** `tos_enforcer`

Le modèle Modérateur est le point de départ recommandé si votre objectif est de réduire la charge de modération manuelle. Il examine les nouveaux commentaires et les commentaires signalés et applique vos règles communautaires.

### Invite initiale intégrée

[inline-code-attrs-start title = 'Invite initiale du modèle de modérateur'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Vous voudrez presque toujours **augmenter cette invite** avec des exemples concrets de ce que votre site autorise ou n'autorise pas. La politique d'escalade propre à la plateforme (avertir avant de bannir, consulter la mémoire avant de bannir) est déjà intégrée dans l'invite système que l'agent reçoit, vous n'avez donc pas besoin de la répéter.

### Déclencheurs

- **New comment posted** (`COMMENT_ADD`) - l'agent examine chaque nouveau commentaire.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - l'agent réévalue un commentaire que d'autres utilisateurs ont signalé.

### Outils autorisés

- [`mark_comment_approved`](#tools-overview) - utile pour les locataires en pré-moderation où l'agent publie les commentaires propres et cache le reste.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Il ne peut pas publier de commentaires, voter, épingler, verrouiller, attribuer des badges ou envoyer des emails - l'invite est volontairement restreinte.

### Ajouts recommandés avant la mise en production

- **Définissez les [Community Guidelines](#community-guidelines).** Quelques phrases de politique écrite suffisent ; l'agent les applique à chaque exécution.
- **Placez `ban_user` derrière une [approbation](#approval-workflow).** C'est activé par défaut dans la région UE (voir [EU DSA Article 17 Compliance](#eu-dsa-compliance)) et recommandé partout.
- **Envisagez également de placer `mark_comment_spam` derrière une approbation** si vous avez un faible volume mais un contenu à enjeux élevés.
- **Placez `mark_comment_approved` derrière une approbation si vous utilisez la pré-moderation.** Approuver un mauvais commentaire le mettra devant les lecteurs ; bloquez cette action jusqu'à ce que l'agent ait gagné la confiance via un dry-run.
- **Cochez "Inclure le facteur de confiance du commentateur, l'ancienneté du compte, l'historique des bannissements et les commentaires récents"** dans [Context Options](#context-options). Le modèle avertira beaucoup moins agressivement lorsqu'il pourra voir qu'une personne est un utilisateur de longue date de bonne foi.

### Période de dry-run recommandée

Exécutez ce modèle en [dry-run](#dry-run-mode) pendant au moins une semaine contre votre trafic réel avant de passer en Enabled. Utilisez [Test Runs (Replays)](#test-runs-replays) pour prévisualiser également sur les 30 jours précédents.

---