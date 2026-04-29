Se déclenche lorsqu'un utilisateur publie son premier commentaire sur ce site (votre tenant). C'est **une seule fois par utilisateur** — les commentaires ultérieurs du même utilisateur ne déclenchent pas à nouveau l'événement.

### Contexte reçu par l'agent

- Le nouveau commentaire.
- Contexte optionnel de fil de discussion / historique de l'utilisateur / page selon la configuration.

Lorsque le contexte d'historique utilisateur est activé, la liste des commentaires récents de l'utilisateur sera bien sûr vide (ou ne contiendra que celui-ci), mais le facteur de confiance et l'ancienneté du compte seront renseignés.

### À noter

- « Premier commentaire sur ce site » est limité au **tenant**, pas à l'ensemble des sites FastComments. Un utilisateur ayant des commentaires sur d'autres sites FastComments déclenchera quand même ce déclencheur la première fois qu'il poste sur le vôtre.
- Le déclencheur ne se produit que pour les utilisateurs disposant d'un userId. Les commentaires anonymes non vérifiés sans userId stable ne le déclenchent pas.
- Le déclencheur se produit lorsque le commentaire est approuvé/visible (pas au moment de la publication initiale). Les modifications ou actions des modérateurs sur les premiers commentaires ne le déclenchent pas à nouveau.

### Cas d'utilisation courants

- **Message de bienvenue** - le [modèle Welcome Greeter](#template-welcome-greeter) est conçu autour de ce déclencheur.
- **Intégration** - envoyer un [DM warning](#tool-warn-user) (utilisé ici comme un rappel amical plutôt qu'un avertissement) orientant l'utilisateur vers les règles de la communauté.
- **Notification au réviseur** - si vous voulez qu'un humain examine le premier message de chaque nouveau commentateur, [`mark_comment_reviewed`](#tools-overview) peut les signaler pour révision.