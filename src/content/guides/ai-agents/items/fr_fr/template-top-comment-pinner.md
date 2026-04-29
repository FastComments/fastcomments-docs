**ID du modèle :** `top_comment_pinner`

L'Épingleur du meilleur commentaire surveille les commentaires de premier niveau qui dépassent un seuil de votes et les épingle — remplaçant ce qui était précédemment épinglé dans le même fil.

L'invite intégrée demande à l'agent d'ignorer les réponses (l'épinglage fonctionne sur les fils, donc épingler une réponse est rarement utile) et de filtrer le contenu promotionnel (pour que l'agent ne favorise pas le spam de liens populaire).

### Déclencheurs

- **Un commentaire dépasse un seuil de votes** (`COMMENT_VOTE_THRESHOLD`, seuil de votes par défaut : 10).

Le déclencheur se produit lorsque le score net du commentaire (`up - down`) atteint le seuil configuré. Ajustez ce nombre dans le formulaire d'édition en fonction de l'activité de vos fils de discussion — 10 est une valeur raisonnable pour des sites modérément actifs.

### Outils autorisés

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

L'épinglage est non destructif — il peut être annulé instantanément — donc ce modèle s'exécute généralement sans approbations.

### Ajouts recommandés avant la mise en production

- **Cochez "Inclure le commentaire parent et les réponses précédentes dans le même fil"** dans [Options de contexte](#context-options). Sans le contexte de fil, l'agent ne peut pas déterminer de manière fiable s'il existe déjà un commentaire épinglé à désépingler.
- **Ajustez le seuil de votes** à la réalité de votre site. Sur des fils très actifs, 10 arrive trop souvent ; sur des fils calmes, 10 peut n'arriver jamais.
- **Envisagez de limiter par URL** si vous ne souhaitez des commentaires épinglés que sur certaines sections de votre site — par exemple les fils d'actualité, mais pas les fils d'annonces.

### Remarque sur l'épinglage en double

L'invite de l'agent lui demande de désépingler d'abord avant d'épingler, mais si le modèle manque cette étape, la plateforme elle-même n'impose pas la règle d'un seul élément épinglé par fil (vous pouvez en avoir plusieurs). Si l'épinglage en double pose problème sur votre site, soumettez `pin_comment` à approbation et révisez chaque épinglage — ou rédigez une invite plus stricte.

---