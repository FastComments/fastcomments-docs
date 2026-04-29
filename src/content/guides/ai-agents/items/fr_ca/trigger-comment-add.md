Déclenche l'agent chaque fois qu'un nouveau commentaire est publié sur une page couverte par la [portée](#scope-url-locale) de l'agent.

### Contexte reçu par l'agent

- Le nouveau commentaire au complet - texte, auteur, votes, ID du parent, ID de l'URL de la page.
- Optionnel : le commentaire parent et les réponses précédentes dans le même fil, si [contexte du fil](#context-options) est activé.
- Optionnel : le facteur de confiance de l'auteur du commentaire, l'âge du compte, l'historique de bannissements et les commentaires récents, si [contexte de l'historique de l'utilisateur](#context-options) est activé.
- Optionnel : les métadonnées de la page, si [contexte de la page](#context-options) est activé.

### À noter

- Le déclencheur s'active **après** l'enregistrement du commentaire. L'agent peut s'y référer directement dans les appels d'outils.
- Il ne se déclenche **pas** pour les commentaires rédigés par un autre agent dans le même locataire.
- Il se déclenche pour les commentaires vérifiés et non vérifiés. Si votre locataire exige l'approbation d'un modérateur avant qu'un commentaire soit visible (voir [How Approvals Work](/guide-moderation.html#moderation-approvals) dans le guide de modération), le déclencheur s'active lorsque le commentaire est créé, et non lorsqu'il est approuvé ultérieurement. Le bot modérateur peut être programmé pour approuver les commentaires pour vous après examen.

### Utilisations courantes

- **Modération** - vérifier le commentaire par rapport aux directives de la communauté, marquer comme spam ou avertir les personnes qui commentent pour la première fois.
- **Message de bienvenue** - bien que [Déclencheur : Premier commentaire d'un nouvel utilisateur](#trigger-new-user-first-comment) soit généralement mieux adapté aux messages de bienvenue, puisqu'il se déclenche une seule fois par utilisateur.
- **Résumé du fil de discussion** - généralement associé à un [délai de déclenchement](#trigger-deferred-delay) afin que le fil se stabilise avant l'exécution de l'agent.

---