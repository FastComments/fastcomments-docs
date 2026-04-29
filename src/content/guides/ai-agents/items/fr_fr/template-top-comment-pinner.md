---
**ID du modèle :** `top_comment_pinner`

The Top Comment Pinner watches for top-level comments that cross a vote threshold and pins them - replacing whatever was pinned previously on the same thread.

### Invite initiale intégrée

[inline-code-attrs-start title = 'Invite initiale du modèle Top Comment Pinner'; type='text' inline-code-attrs-end]
[inline-code-start]
Vous épinglez les meilleurs commentaires de premier niveau d'un fil. Lorsqu'un commentaire atteint le seuil de votes, épinglez-le si son contenu est substantiel et non promotionnel. Désépingler d'abord tout commentaire épinglé précédemment sur le même fil. N'épinglez pas les réponses, seulement les commentaires de premier niveau.
[inline-code-end]

The "do not pin replies" instruction matters: pinning works on threads, so pinning a reply is rarely useful. The "non-promotional" filter keeps the agent from boosting a popular link-spam comment.

### Déclencheurs

- **Un commentaire dépasse un seuil de votes** (`COMMENT_VOTE_THRESHOLD`, seuil de votes par défaut : 10).

The trigger fires when the comment's net votes (`up - down`) reaches the configured threshold. Tune the number on the edit form based on how active your threads are - 10 is a sensible default for moderately active sites.

### Outils autorisés

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pinning is non-destructive - it can be reversed instantly - so this template usually runs without approvals.

### Ajouts recommandés avant la mise en production

- **Cochez "Inclure le commentaire parent et les réponses précédentes dans le même fil"** dans [Options de contexte](#context-options). Sans le contexte du fil, l'agent ne peut pas déterminer de manière fiable s'il existe déjà un commentaire épinglé à désépingler.
- **Ajustez le seuil de votes** pour votre site. Sur des fils très actifs, 10 arrive trop souvent ; sur des fils calmes, 10 peut n'arriver jamais.
- **Envisagez de limiter par URL** si vous ne souhaitez des commentaires épinglés que dans certaines sections de votre site - par exemple les fils d'actualité, mais pas les fils d'annonces.

### Remarque sur l'épinglage en double

The agent's prompt instructs it to unpin first before pinning, but if the model misses that step the platform itself does not enforce a one-pinned-per-thread rule (you can have multiple). If duplicate pinning is a problem on your site, gate `pin_comment` behind approval and review each one - or write a tighter prompt.

---