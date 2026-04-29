---
Se déclenche lorsqu'un modérateur attribue un badge à un utilisateur.

### Contexte reçu par l'agent

- L'**ID du badge** attribué.
- L'**ID de l'utilisateur déclencheur** - le modérateur qui a attribué le badge.
- Contexte optionnel de fil de discussion / historique utilisateur / page tel que configuré.

Le site d'origine n'inclut **pas** de `commentId` dans la charge utile du déclencheur, même si le badge a été attribué par rapport à un commentaire spécifique.

### Qui déclenche cet événement

Une action d'un modérateur humain.

### Remarques

- Seul l'ID du badge est inclus ; l'agent ne reçoit pas les métadonnées du badge (nom, image). Si l'agent a besoin de déterminer *quel* badge a été attribué, intégrez ce contexte dans le [initial prompt](#personality-prompt) ou les [directives de la communauté](#community-guidelines).
- Le déclencheur se produit une fois par attribution de badge, et non par utilisateur. Attribuer le même badge à un utilisateur deux fois le déclenche deux fois (chaque attribution est un événement distinct).

### Usages courants

- **Reconnaissance réciproque** - un agent peut publier une réponse "merci pour cette excellente contribution" lorsqu'un badge spécifique est attribué.
- **Flux de reconnaissance externe** via [Webhooks](#webhooks-overview) - répliquez les attributions de badges dans votre propre système d'engagement utilisateur.
- **Enregistrement en mémoire** - des notes telles que "cet utilisateur est un contributeur reconnu" devraient être prises en compte par de futurs agents de modération dans leurs décisions.

---