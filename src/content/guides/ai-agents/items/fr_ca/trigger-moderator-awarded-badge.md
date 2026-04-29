Se déclenche lorsqu'un modérateur attribue un badge à un utilisateur.

### Contexte que reçoit l'agent

- L'**ID du badge** attribué.
- L'**ID de l'utilisateur déclencheur** - le modérateur qui a attribué le badge.
- Contexte optionnel du fil de discussion / historique de l'utilisateur / page, selon la configuration.

Le site de déclenchement n'inclut **pas** de `commentId` dans la charge utile du déclencheur, même si le badge a été attribué par rapport à un commentaire spécifique.

### Qui déclenche cet événement

Une action effectuée par un modérateur humain.

### À noter

- Seul l'ID du badge est inclus ; l'agent ne reçoit pas les métadonnées du badge (nom, image). Si l'agent doit déterminer *quel* badge a été attribué, incorporez ce contexte dans l'[invite initiale](#personality-prompt) ou les [directives communautaires](#community-guidelines).
- Le déclencheur se produit une fois par attribution de badge, et non par utilisateur. Attribuer le même badge deux fois à un utilisateur déclenchera l'événement deux fois (chaque attribution est un événement distinct).

### Utilisations courantes

- **Reconnaissance réciproque** - un agent peut publier une réponse « merci pour cette excellente contribution » lorsqu'un badge spécifique est attribué.
- **Flux de reconnaissance externe** via [Webhooks](#webhooks-overview) - répliquer les attributions de badges dans votre propre système d'engagement des utilisateurs.
- **Enregistrement en mémoire** - des notes « cet utilisateur est un contributeur reconnu » pour que les futurs agents de modération en tiennent compte dans leurs décisions.

---