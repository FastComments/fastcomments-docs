---
Se déclenche lorsqu'un modérateur approuve un commentaire.

### Contexte que reçoit l'agent

- Le commentaire nouvellement approuvé.
- L'**ID utilisateur déclencheur** - le modérateur qui a approuvé.
- Historique facultatif du fil / de l'utilisateur / contexte de la page selon la configuration.

### Qui déclenche cet événement

Une action d'un modérateur humain.

### À noter

- Un commentaire "approuvé" est un commentaire **visible** dans la terminologie FastComments. Voir [Comment fonctionnent les approbations](/guide-moderation.html#moderation-approvals) dans le guide de modération pour la distinction entre approuvé/non approuvé et examiné/non examiné.
- Le déclencheur se produit lors des **transitions** d'approbation : un commentaire passant de non approuvé à approuvé le déclenche ; un commentaire déjà approuvé qui est enregistré à nouveau ne le déclenche pas.
- Pour les locataires où les commentaires sont par défaut approuvés automatiquement, ce déclencheur ne se produit que lorsqu'un modérateur réapprouve explicitement un commentaire précédemment masqué.

### Utilisations courantes

- **Bienvenue / engagement** - un agent peut répondre aux commentateurs pour la première fois au moment où un modérateur les approuve, plutôt qu'au moment de la publication.
- **Coordination entre agents** - si un autre agent avait marqué le commentaire pour examen, l'approbation est le signal que la révision humaine est terminée.
- **Piste d'audit** via [Webhooks](#webhooks-overview).

---