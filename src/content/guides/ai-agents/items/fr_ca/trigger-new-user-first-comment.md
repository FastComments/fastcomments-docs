Se déclenche lorsqu'un utilisateur publie son premier commentaire sur ce site (votre locataire). C'est **une seule fois par utilisateur** - les commentaires suivants du même utilisateur ne le déclenchent pas à nouveau.

### Contexte que reçoit l'agent

- Le nouveau commentaire.
- Contexte optionnel : fil de discussion / historique de l'utilisateur / page, tel que configuré.

Lorsque le contexte de l'historique utilisateur est activé, la liste des commentaires récents de l'utilisateur sera bien sûr vide (ou contiendra uniquement celui-ci), mais le facteur de confiance et l'ancienneté du compte sont renseignés.

### À noter

- « Premier commentaire sur ce site » est limité au **locataire**, pas à l'échelle de tous les sites FastComments. Un utilisateur ayant des commentaires sur d'autres sites FastComments déclenchera quand même ce déclencheur la première fois qu'il publie sur le vôtre.
- Le déclencheur ne se produit que pour les utilisateurs disposant d'un userId. Les commentaires anonymes non vérifiés sans userId stable ne le déclenchent pas.
- Le déclencheur se produit lorsque le commentaire est approuvé/visible (et non au moment de la publication initiale). Les modifications ou actions des modérateurs sur les premiers commentaires ne le déclenchent pas à nouveau.

### Utilisations courantes

- **Message de bienvenue** - le [modèle Welcome Greeter](#template-welcome-greeter) est construit autour de ce déclencheur.
- **Intégration** - envoyer un [avertissement DM](#tool-warn-user) (utilisé ici comme un rappel amical plutôt qu'un avertissement) orientant l'utilisateur vers les règles de la communauté.
- **Notification au réviseur** - si vous voulez qu'un humain examine le premier message de chaque nouveau commentateur, [`mark_comment_reviewed`](#tools-overview) peut les marquer pour révision.