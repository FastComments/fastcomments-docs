---
Un sous-ensemble d'actions de modération peut être effectué directement depuis le fil de commentaires lui-même, sans avoir à aller sur la page de modération des commentaires.

Lorsque vous êtes connecté, cliquez sur le bouton d'édition en haut à droite d'un commentaire. En tant que modérateur, vous devez avoir les options suivantes :

- **Épingler** ce commentaire
- **Supprimer** ce commentaire
- **Supprimer** ce commentaire + **Bannir l'utilisateur** (Permanent ou Shadow, plus de détails plus tard)
- **Modifier** ce commentaire
- Marquer ce commentaire comme **Approuvé** (l'afficher) ou **Non approuvé** (le masquer)
- Marquer ce commentaire comme **Spam** ou **Non spam**

### Fermeture des fils de commentaires

Les modérateurs et les administrateurs peuvent verrouiller, ou fermer, les fils de commentaires, en sélectionnant `Close Thread` dans le menu à trois points en haut de la zone de commentaires, s'ils sont connectés. Ils peuvent sélectionner `Re-Open Thread` plus tard, à tout moment, pour rouvrir la possibilité de commenter.

La fermeture d'un fil de commentaires empêche les nouveaux commentaires, mais permet toujours de voter et aux utilisateurs de supprimer leurs commentaires s'ils le souhaitent.

La fermeture et la réouverture des fils de commentaires affectent instantanément tous les utilisateurs qui consultent le fil.

Vous pouvez également marquer un fil en lecture seule, ce qui supprime également les options de vote et de suppression, en créant une règle de personnalisation spécifiquement pour cette page.

### Mise à jour en direct

Toutes ces actions mettront à jour immédiatement les fils de commentaires des autres utilisateurs sans qu'ils aient à recharger la page. Cependant, les actions de modération comme masquer un commentaire ou le signaler comme spam ne suppriment pas le commentaire de **l'écran du modérateur** afin que, si nécessaire, celui-ci puisse rapidement annuler l'action. Pour indiquer qu'un commentaire est masqué, il sera mis en évidence par rapport aux autres commentaires (la couleur du surlignage dépendant de la raison de la suppression).

Par exemple, étant donné les utilisateurs `A (commenter)`, `B (Moderator 1)`, et `C (Moderator 2)`.

...et le scénario suivant :

1. `User B (Moderator 1)` masque un commentaire.
2. Pour `User A (commenter)`, ce commentaire est immédiatement masqué.
3. Pour `User C (Moderator 2)`, ce commentaire est immédiatement masqué.
4. Pour l'utilisateur qui a effectué la modification, `User B (Moderator 1)`, le commentaire reste sur son écran, mais est mis en évidence comme supprimé. Celui-ci a la possibilité d'annuler son action, auquel cas les autres utilisateurs verront à nouveau la mise à jour en direct.

---