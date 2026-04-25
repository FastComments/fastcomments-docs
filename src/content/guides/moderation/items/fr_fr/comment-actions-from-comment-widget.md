Un sous-ensemble d'actions de modération peut être effectué directement depuis le fil de commentaires lui-même, sans avoir à se rendre sur la page de modération des commentaires.

Lorsque vous êtes connecté, cliquez sur le bouton d'édition en haut à droite d'un commentaire. En tant que modérateur, vous devriez avoir les options suivantes :

- **Épingler** ce commentaire
- **Supprimer** ce commentaire
- **Supprimer** ce commentaire + **Bannir l'utilisateur** (Permanent ou Shadow, plus de détails plus loin)
- **Modifier** ce commentaire
- **Verrouiller** ou **Déverrouiller** ce commentaire (plus de détails ci-dessous)
- Marquer ce commentaire **Approuvé** (l'afficher) ou **Non approuvé** (le masquer)
- Marquer ce commentaire comme **Spam** ou **Non Spam**

### Verrouillage d'un commentaire

Le verrouillage d'un commentaire individuel empêche toute nouvelle réponse à celui-ci, et empêche également que le commentaire lui-même soit modifié ou supprimé tant qu'il n'est pas déverrouillé. Cela s'applique à tout le monde, y compris aux administrateurs et aux modérateurs. Si vous devez modifier ou supprimer un commentaire verrouillé, déverrouillez-le d'abord, effectuez la modification, puis reverrouillez-le si souhaité.

Une icône de cadenas apparaît dans le coin supérieur droit d'un commentaire verrouillé afin que les lecteurs puissent voir en un coup d'œil que le fil est fermé. Les entrées de menu Modifier et Supprimer sont masquées pour les commentaires verrouillés à la fois dans le widget de commentaires et dans l'API publique (`PATCH` et `DELETE` renvoient `code: 'locked'` si elles sont appelées sur un commentaire verrouillé).

Deux exceptions intentionnelles contournent le verrou, car autrement elles laisseraient des données orphelines : lorsqu'un utilisateur supprime l'intégralité de son compte (ses commentaires sont nettoyés quel que soit l'état du verrou), et lorsqu'un modérateur bannit un utilisateur avec l'option "delete all comments from this user" (le balayage supprime les verrous).

### Fermeture des fils de commentaires

Les modérateurs et les administrateurs peuvent verrouiller, ou fermer, les fils de commentaires en sélectionnant `Close Thread` dans le menu à trois points en haut de la zone de commentaires, s'ils sont connectés. Ils peuvent sélectionner `Re-Open Thread` plus tard, à tout moment, pour rouvrir les commentaires.

Fermer un fil de commentaires empêche les nouveaux commentaires, mais permet toujours de voter et aux utilisateurs de supprimer leurs commentaires s'ils le souhaitent.

La fermeture et la réouverture des fils de commentaires affectent instantanément tous les utilisateurs qui consultent le fil.

Vous pouvez également marquer un fil en lecture seule, ce qui supprime également les options de vote et de suppression, en créant une règle de personnalisation spécifiquement pour cette page.

### Mise à jour en temps réel

Toutes ces actions mettront à jour les fils de commentaires des autres utilisateurs immédiatement sans qu'ils aient à recharger la page. Cependant, les actions de modération comme masquer un commentaire ou le marquer comme spam, ne retirent pas le commentaire de **l'écran du modérateur** afin qu'il puisse, si nécessaire, annuler rapidement l'action. Pour indiquer qu'un commentaire est masqué, il sera mis en évidence par rapport aux autres commentaires (la couleur de mise en évidence dépendant de la raison de la suppression).

Par exemple, pour les utilisateurs `A (commenter)`, `B (Moderator 1)`, et `C (Moderator 2)`.

...et le scénario suivant :

1. `User B (Moderator 1)` masque un commentaire.
2. Pour `User A (commenter)` ce commentaire est immédiatement masqué.
3. Pour `User C (Moderator 2)` ce commentaire est immédiatement masqué.
4. Pour l'utilisateur qui a effectué le changement, `User B (Moderator 1)`, le commentaire reste sur son écran, mais est mis en évidence comme supprimé. Il a la possibilité d'annuler son action, auquel cas les autres utilisateurs verront la mise à jour, à nouveau en temps réel.

---