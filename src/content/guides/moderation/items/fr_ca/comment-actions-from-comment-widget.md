Un sous-ensemble d'actions de modération peut être effectué directement depuis le fil de commentaires lui-même, sans avoir à aller à la page de modération des commentaires.

Lorsque vous êtes connecté, cliquez sur le bouton d'édition en haut à droite d'un commentaire. Vous devriez avoir les options suivantes en tant que modérateur :

- **Épingler** ce commentaire
- **Supprimer** ce commentaire
- **Supprimer** ce commentaire + **Bannir l'utilisateur** (Permanent ou Shadow, plus de détails plus tard)
- **Modifier** ce commentaire
- **Verrouiller** ou **Déverrouiller** ce commentaire (plus de détails ci-dessous)
- Marquer ce commentaire **Approuvé** (l'afficher) ou **Non approuvé** (le masquer)
- Marquer ce commentaire comme **Spam** ou **Non spam**

### Verrouillage d'un commentaire

Le verrouillage d'un commentaire individuel empêche toute nouvelle réponse à celui-ci, et empêche également que le commentaire lui-même soit modifié ou supprimé tant qu'il n'est pas déverrouillé. Cela s'applique à tous, y compris aux administrateurs et aux modérateurs. Si vous devez modifier ou supprimer un commentaire verrouillé, déverrouillez-le d'abord, effectuez la modification, puis reverrouillez-le si souhaité.

Une icône de cadenas apparaît dans le coin supérieur droit d'un commentaire verrouillé afin que les lecteurs puissent voir d'un coup d'œil que le fil est fermé. Les entrées de menu Modifier et Supprimer sont masquées pour les commentaires verrouillés dans le widget de commentaires et dans l'API publique (`PATCH` et `DELETE` renvoient `code: 'locked'` s'ils sont appelés contre un commentaire verrouillé).

Deux exceptions intentionnelles contournent le verrouillage, car elles laisseraient sinon des données orphelines : lorsqu'un utilisateur supprime son compte entier (ses commentaires sont nettoyés indépendamment de l'état du verrouillage), et lorsqu'un modérateur bannit un utilisateur avec l'option "supprimer tous les commentaires de cet utilisateur" (le balayage supprime les verrous).

### Fermeture d'un fil de commentaires

Les modérateurs et les administrateurs peuvent verrouiller, ou fermer, des fils de commentaires, en sélectionnant `Close Thread` dans le menu à trois points en haut de la zone de commentaires, s'ils sont connectés. Ils peuvent sélectionner `Re-Open Thread` plus tard, à tout moment, pour rouvrir les commentaires.

La fermeture d'un fil de commentaires empêche la publication de nouveaux commentaires, mais permet toujours les votes et aux utilisateurs de supprimer leurs commentaires si désiré.

Vous pouvez également rendre un fil en lecture seule, ce qui supprime également les options de vote et de suppression, en créant une règle de personnalisation spécifiquement pour cette page.

### Mises à jour en direct

Toutes ces actions mettront à jour les fils de commentaires des autres utilisateurs immédiatement, sans qu'ils aient à recharger la page. Cependant, les actions de modération comme masquer un commentaire ou le marquer comme spam n'enlèvent pas le commentaire de **l'écran du modérateur** afin que, si nécessaire, il puisse rapidement annuler l'action. Pour indiquer qu'un commentaire est masqué, il sera mis en évidence par rapport aux autres commentaires (la couleur de mise en évidence dépendant de la raison de la suppression).

Par exemple, étant donné les utilisateurs `A (commenter)`, `B (Moderator 1)`, et `C (Moderator 2)`.

...et le scénario suivant :

1. `User B (Moderator 1)` masque un commentaire.
2. Pour `User A (commenter)` ce commentaire est immédiatement masqué.
3. Pour `User C (Moderator 2)` ce commentaire est immédiatement masqué.
4. Pour l'utilisateur qui a effectué la modification, `User B (Moderator 1)`, le commentaire reste sur son écran, mais est mis en évidence comme retiré. Il a la possibilité d'annuler son action, auquel cas les autres utilisateurs verront la mise à jour en direct, de nouveau.