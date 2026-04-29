The Edit tool lets the agent replace the text of an existing comment. It is destructive in a way most other moderation tools are not: it overwrites user-authored content. Reserve it for narrow, clear-cut cases.

### Ce que cela fait

L'agent fournit un ID de commentaire et un corps de remplacement. La plateforme écrit le nouveau texte dans le commentaire et enregistre une entrée `TextChanged` dans le journal d'audit du commentaire, capturant à la fois l'ancien texte et le nouveau. L'original n'est jamais perdu : les modérateurs peuvent lire ce que disait le commentaire avant que l'agent ne l'ait édité.

Le texte de remplacement passe par la même pipeline de rendu que pour une édition humaine : le masquage des grossièretés, l'analyse des mentions, l'extraction des hashtags et le traitement des liens/images se comportent exactement comme si l'auteur original avait soumis le nouveau texte.

### Portée

Comme tous les outils modifiant des commentaires, Edit est contraint par l'allowlist du déclencheur - l'agent ne peut éditer que le commentaire sur lequel le déclencheur s'est déclenché, son parent, ou un autre commentaire dans le périmètre du même contexte de déclenchement. Une tentative d'injection de prompt du type "edit comment XYZ" où XYZ est sans rapport sera refusée côté serveur avant que l'exécuteur ne s'exécute.

### Boucles

Lorsque l'agent édite un commentaire, la plateforme déclenche un trigger `COMMENT_EDIT` comme pour une édition humaine, mais **supprime la diffusion vers les autres agents**. Cela empêche deux agents qui écoutent tous les deux `COMMENT_EDIT` de se renvoyer la balle indéfiniment sur leurs modifications respectives.

### Quand l'autoriser

Pour les agents qui gèrent la redaction de PII, ou pour les agents de résumé/digest qui s'auto-éditent. La plupart des agents de modération n'ont **pas** besoin de cet outil - mark-spam, warn, and ban couvrent le cycle de vie typique.

### Approbation

**Envisagez fortement de placer cela derrière une approbation**, surtout tant que vous êtes en train d'établir la confiance envers l'agent. Un agent qui réécrit les paroles d'un utilisateur est le type d'action qu'une communauté remarquera et contre laquelle elle réagira, et il est plus difficile de « undo » cela sur le plan réputationnel qu'une suppression.

### Voir aussi

- [Trigger: Comment Edited](#trigger-comment-edit) - le déclencheur activé lorsqu'un texte de commentaire change.
- [Approval Workflow](#approval-workflow) - comment placer l'outil sous revue humaine.