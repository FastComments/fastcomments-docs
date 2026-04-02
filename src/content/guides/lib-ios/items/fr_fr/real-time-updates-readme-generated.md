Après l'appel de `sdk.load()`, le SDK s'abonne automatiquement aux événements WebSocket pour le `urlId` configuré. Les événements suivants sont gérés :

- Nouveaux commentaires, modifications et suppressions
- Votes (ajoutés et retirés)
- Modifications des états d'épinglage, de verrouillage, de signalement et de blocage
- Présence des utilisateurs (connexion/déconnexion)
- Ouverture/fermeture de fil de discussion
- Attribution de badges
- Mises à jour de la configuration du serveur

### Contrôle de l'affichage en direct

Par défaut, les nouveaux commentaires des autres utilisateurs apparaissent immédiatement :

```swift
sdk.showLiveRightAway = true   // Par défaut : afficher immédiatement
```

Réglez ceci sur `false` pour mettre en tampon les nouveaux commentaires derrière un bouton "N nouveaux commentaires", permettant à l'utilisateur de choisir quand les afficher :

```swift
sdk.showLiveRightAway = false
```

### Présence des utilisateurs

Des indicateurs en ligne/hors ligne apparaissent automatiquement sur les avatars des utilisateurs lorsque le serveur active le suivi de présence. Aucune configuration supplémentaire n'est nécessaire côté client.

---
---