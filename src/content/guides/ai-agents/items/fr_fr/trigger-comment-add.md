---
Déclenche l'agent à chaque fois qu'un nouveau commentaire est publié sur une page couverte par le [périmètre](#scope-url-locale) de l'agent.

### Contexte que l'agent reçoit

- Le nouveau commentaire dans son intégralité - texte, auteur, votes, ID du parent, ID de l'URL de la page.
- Optionnel : le commentaire parent et les réponses précédentes du même fil, si le [contexte du fil](#context-options) est activé.
- Optionnel : le facteur de confiance du commentateur, l'ancienneté du compte, l'historique de bannissements et les commentaires récents, si le [contexte de l'historique utilisateur](#context-options) est activé.
- Optionnel : les métadonnées de la page, si le [contexte de la page](#context-options) est activé.

### Remarques importantes

- Le déclencheur se produit **après** que le commentaire a été enregistré. L'agent peut s'y référer directement dans les appels d'outils.
- Il ne se déclenche **pas** pour les commentaires rédigés par un autre agent dans le même locataire.
- Il se déclenche pour les commentaires vérifiés et non vérifiés. Si votre locataire exige l'approbation d'un modérateur avant qu'un commentaire ne soit visible (voir [Comment fonctionnent les approbations](/guide-moderation.html#moderation-approvals) dans le guide de modération), le déclencheur se produit lorsque le commentaire est créé, pas lorsqu'il est ensuite approuvé. Le bot modérateur peut être instruit pour approuver les commentaires pour vous après examen.

### Utilisations courantes

- **Modération** - vérifier le commentaire par rapport aux directives de la communauté, marquer comme spam ou avertir les nouveaux utilisateurs.
- **Message de bienvenue** - bien que [Déclencheur : Premier commentaire d'un nouvel utilisateur](#trigger-new-user-first-comment) soit généralement plus adapté pour les messages de bienvenue puisqu'il se déclenche une fois par utilisateur.
- **Résumé du fil** - généralement associé à un [délai du déclencheur](#trigger-deferred-delay) afin que le fil se stabilise avant l'exécution de l'agent.

---