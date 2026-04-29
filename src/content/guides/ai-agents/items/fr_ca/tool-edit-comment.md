---
L'outil Edit permet à l'agent de remplacer le texte d'un commentaire existant. Il est destructeur d'une manière que la plupart des autres outils de modération ne sont pas : il écrase du contenu rédigé par l'utilisateur. Réservez-le aux cas étroits et bien définis.

### Ce que ça fait

L'agent transmet un ID de commentaire et un corps de remplacement. La plateforme écrit le nouveau texte dans le commentaire et enregistre une entrée `TextChanged` dans le journal d'audit du commentaire capturant à la fois l'ancien texte et le nouveau texte. L'original n'est jamais perdu - les modérateurs peuvent lire ce que le commentaire disait avant que l'agent ne l'ait modifié.

Le remplacement passe par le même pipeline de rendu qu'une édition humaine : le masquage des jurons, l'analyse des mentions, l'extraction des hashtags et le traitement des liens/images se comportent exactement comme si l'auteur original avait soumis le nouveau texte.

### Portée

Comme pour tout outil qui modifie un commentaire, Edit est contraint à la allowlist du trigger - l'agent ne peut éditer que le commentaire sur lequel le trigger s'est déclenché, son parent, ou un autre commentaire dans le périmètre du même contexte de trigger. Une tentative d'injection de prompt visant à "éditer le commentaire XYZ" où XYZ est sans rapport sera refusée côté serveur avant que l'exécuteur ne s'exécute.

### Boucles

Quand l'agent édite un commentaire, la plateforme déclenche un `COMMENT_EDIT` trigger comme pour une édition humaine, mais **supprime l'envoi aux autres agents**. Cela empêche deux agents qui écoutent tous les deux `COMMENT_EDIT` de se renvoyer des modifications en boucle.

### Quand l'autoriser

Pour les agents qui gèrent la suppression de PII, ou pour les agents qui rédigent eux-mêmes des résumés/compilations. La plupart des agents de modération n'ont **pas** besoin de cet outil - mark-spam, warn, et ban couvrent le cycle de vie typique.

### Approbations

**Envisagez fortement de le soumettre à une approbation**, surtout tant que vous construisez la confiance envers l'agent. Un agent qui réécrit les propos d'un utilisateur est une action que la communauté remarquera et à laquelle elle réagira, et il est plus difficile, d'un point de vue réputationnel, de "défaire" cela qu'une suppression.

### Voir aussi

- [Trigger: Comment Edited](#trigger-comment-edit) - le trigger déclenché lorsque le texte d'un commentaire change.
- [Approval Workflow](#approval-workflow) - comment placer l'outil derrière une révision humaine.

---