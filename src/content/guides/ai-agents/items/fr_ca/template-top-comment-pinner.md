---
**ID du modèle :** `top_comment_pinner`

Le Top Comment Pinner surveille les commentaires de premier niveau qui dépassent un seuil de votes et les épingle — remplaçant ce qui était précédemment épinglé dans le même fil.

L'invite intégrée demande à l'agent d'ignorer les réponses (l'épinglage fonctionne sur les fils, donc épingler une réponse est rarement utile) et de filtrer le contenu promotionnel (pour que l'agent ne favorise pas le spam de liens populaire).

### Triggers

- **Un commentaire dépasse un seuil de votes** (`COMMENT_VOTE_THRESHOLD`, seuil de votes par défaut : 10).

Le déclencheur se produit lorsque le nombre net de votes du commentaire (`up - down`) atteint le seuil configuré. Ajustez ce nombre dans le formulaire d'édition en fonction de l'activité de vos fils de discussion - 10 est une valeur par défaut raisonnable pour des sites modérément actifs.

### Allowed tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

L'épinglage n'est pas destructif - il peut être annulé instantanément - donc ce modèle s'exécute généralement sans approbation.

### Recommended additions before going live

- **Cochez "Include parent comment and prior replies in the same thread"** dans [Options de contexte](#context-options). Sans le contexte du fil, l'agent ne peut pas déterminer de manière fiable s'il existe déjà un commentaire épinglé à désépingler.
- **Ajustez le seuil de votes** pour votre site. Sur des fils très actifs, 10 se produit trop souvent ; sur des fils calmes, 10 peut ne jamais être atteint.
- **Envisagez de limiter par URL** si vous ne voulez des commentaires épinglés que dans certaines sections de votre site — par exemple les fils d'actualités, mais pas les fils d'annonces.

### Note on duplicate pinning

L'invite de l'agent lui demande de désépingler d'abord avant d'épingler, mais si le modèle omet cette étape, la plateforme elle-même n'applique pas de règle d'un seul épinglage par fil (vous pouvez en avoir plusieurs). Si l'épinglage en double est un problème sur votre site, conditionnez `pin_comment` à une approbation et examinez chacun d'eux - ou rédigez une invite plus stricte.

---