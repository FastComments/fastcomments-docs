**ID du modèle:** `top_comment_pinner`

Le Top Comment Pinner surveille les commentaires de niveau supérieur qui dépassent un seuil de votes et les épingle - remplaçant ce qui était épinglé auparavant dans le même fil.

### Invite initiale intégrée

[inline-code-attrs-start title = 'Invite initiale du modèle Top Comment Pinner'; type='text' inline-code-attrs-end]
[inline-code-start]
Vous épinglez les meilleurs commentaires de niveau supérieur d'un fil. Lorsqu'un commentaire atteint le seuil de votes, épinglez-le si son contenu est substantiel et non promotionnel. Désépinglez d'abord tout commentaire précédemment épinglé dans le même fil. N'épinglez pas les réponses, seulement les commentaires de niveau supérieur.
[inline-code-end]

L'instruction "ne pas épingler les réponses" est importante : l'épinglage fonctionne par fil, donc épingler une réponse est rarement utile. Le filtre "non promotionnel" empêche l'agent de mettre en avant un commentaire spam de liens populaire.

### Déclencheurs

- **Un commentaire dépasse un seuil de votes** (`COMMENT_VOTE_THRESHOLD`, seuil de votes par défaut : 10).

Le déclencheur se déclenche lorsque le total net de votes du commentaire (`up - down`) atteint le seuil configuré. Ajustez ce nombre dans le formulaire d'édition selon l'activité de vos fils de discussion - 10 est une valeur raisonnable pour des sites modérément actifs.

### Outils autorisés

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

L'épinglage n'est pas destructif - il peut être annulé instantanément - donc ce modèle s'exécute généralement sans approbations.

### Ajouts recommandés avant la mise en ligne

- **Cochez "Inclure le commentaire parent et les réponses antérieures dans le même fil"** dans [Options de contexte](#context-options). Sans le contexte du fil, l'agent ne peut pas déterminer de manière fiable s'il y a déjà un commentaire épinglé à désépingler.
- **Ajustez le seuil de votes** pour votre site. Sur des fils très actifs, 10 se produit trop souvent ; sur des fils calmes, 10 peut ne jamais être atteint.
- **Envisagez de restreindre par URL** si vous voulez des commentaires épinglés seulement dans certaines sections de votre site - par exemple les fils d'actualité, mais pas les fils d'annonce.

### Remarque sur l'épinglage en double

L'invite de l'agent lui demande de désépingler d'abord avant d'épingler, mais si le modèle omet cette étape, la plateforme elle-même n'impose pas la règle d'un seul épinglage par fil (vous pouvez en avoir plusieurs). Si l'épinglage en double est un problème sur votre site, placez `pin_comment` derrière une approbation et examinez chaque épinglage - ou rédigez une invite plus stricte.

---