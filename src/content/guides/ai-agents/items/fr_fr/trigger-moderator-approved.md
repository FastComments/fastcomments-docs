Se déclenche lorsqu'un modérateur approuve un commentaire.

### Contexte que reçoit l'agent

- Le commentaire nouvellement approuvé.
- The **triggering user ID** - le modérateur qui a approuvé.
- Contexte optionnel de fil / historique utilisateur / page tel que configuré.

### Qui déclenche cet événement

Une action d'un modérateur humain.

### À noter

- Un commentaire "approuvé" est un commentaire **visible** dans la terminologie FastComments. Voir [Comment fonctionnent les approbations](/guide-moderation.html#moderation-approvals) dans le guide de modération pour la distinction entre approuvé/non approuvé et examiné/non examiné.
- Le déclencheur se produit lors des **transitions** d'approbation : un commentaire passant de non approuvé à approuvé le déclenche ; un commentaire déjà approuvé réenregistré ne le fait pas.
- Pour les tenants où les commentaires sont par défaut auto-approuvés, ce déclencheur ne se produit que lorsqu'un modérateur réapprouve explicitement un commentaire précédemment masqué.

### Utilisations courantes

- **Accueil / engagement** - un agent peut répondre aux commentateurs pour la première fois au moment où un modérateur les approuve, plutôt qu'au moment de la publication.
- **Coordination entre agents** - si un agent distinct avait marqué le commentaire pour examen, l'approbation est le signal que l'examen humain est terminé.
- **Piste d'audit** via [Webhooks](#webhooks-overview).

---