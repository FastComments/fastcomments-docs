---
**Template ID:** `tos_enforcer`

Le modèle Moderator est le point de départ recommandé si votre objectif est de réduire la charge de modération manuelle. Il passe en revue les nouveaux commentaires et ceux signalés, et applique vos règles communautaires.

Vous voudrez presque toujours **compléter l'invite intégrée** avec des exemples concrets de ce que votre site autorise et n'autorise pas. La politique d'escalade propre à la plateforme (avertir avant de bannir, consulter la mémoire avant de bannir) est déjà intégrée à l'invite système que reçoit l'agent, vous n'avez donc pas besoin de la répéter.

### Triggers

- **New comment posted** (`COMMENT_ADD`) - l'agent examine chaque nouveau commentaire.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - l'agent réévalue un commentaire que d'autres utilisateurs ont signalé.

### Allowed tools

- [`mark_comment_approved`](#tools-overview) - utile pour les locataires en pré-moderation où l'agent publie les commentaires propres et masque les autres.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Il ne peut pas publier de commentaires, voter, épingler, verrouiller, attribuer des badges, ou envoyer des courriels - l'invite est intentionnellement restreinte.

### Recommended additions before going live

- **Set [règles communautaires](#community-guidelines).** Quelques phrases de politique écrite suffisent ; l'agent les applique à chaque exécution.
- **Gate `ban_user` behind [approval](#approval-workflow).** Ceci est activé par défaut dans la région UE (voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance)) et est recommandé partout.
- **Consider also gating `mark_comment_spam` behind approval** si vous avez du contenu à faible volume mais à enjeu élevé.
- **Gate `mark_comment_approved` behind approval if you run pre-moderation.** Approuver un mauvais commentaire le met devant les lecteurs ; restreignez cette action jusqu'à ce que l'agent ait gagné la confiance via le mode dry-run.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** in [Options de contexte](#context-options). Le modèle avertira beaucoup moins agressivement lorsqu'il pourra voir qu'une personne est un utilisateur de bonne foi de longue date.

### Recommended dry-run window

Exécutez ce modèle en [mode dry-run](#dry-run-mode) pendant au moins une semaine contre votre trafic réel avant de passer à Activé. Utilisez [Test Runs (Replays)](#test-runs-replays) pour également prévisualiser les 30 jours précédents.

---