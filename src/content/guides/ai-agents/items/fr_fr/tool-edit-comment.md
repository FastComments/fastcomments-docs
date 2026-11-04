L'outil Edit permet à l'agent de remplacer le texte d'un commentaire existant. Il est destructif d'une manière que la plupart des autres outils de modération ne sont pas : il écrase le contenu rédigé par l'utilisateur. Réservez-le à des cas étroits et sans ambiguïté.

### Ce qu'il fait

L'agent transmet un ID de commentaire et un nouveau corps de texte. La plateforme écrit le nouveau texte dans le commentaire et enregistre une entrée `TextChanged` dans le journal d'audit du commentaire, capturant à la fois l'ancien texte et le nouveau. L'original n'est jamais perdu - les modérateurs peuvent lire ce que disait le commentaire avant que l'agent ne le modifie.

Le remplacement passe par le même pipeline de rendu qu'une modification humaine : le masquage des grossièretés, l'analyse des mentions, l'extraction des hashtags et la gestion des liens/images se comportent exactement comme si l'auteur original avait soumis le nouveau texte.

### Périmètre

Comme tous les outils modifiant des commentaires, Edit est contraint par la liste blanche du déclencheur - l'agent ne peut modifier que le commentaire sur lequel le déclencheur s'est déclenché, son parent, ou un autre commentaire dans le périmètre du même contexte de déclencheur. Une tentative d'injection de prompt pour « modifier le commentaire XYZ » où XYZ n'est pas concerné sera refusée côté serveur avant l'exécution de l'executor.

### Boucles

Lorsque l'agent modifie un commentaire, la plateforme déclenche un trigger `COMMENT_EDIT` comme elle le ferait pour une modification humaine, mais **supprime la distribution aux autres agents**. Cela empêche deux agents qui écoutent tous deux `COMMENT_EDIT` de se renvoyer la balle indéfiniment à cause des modifications de l'un et de l'autre.

### Quand l'autoriser

Pour les agents qui gèrent la suppression de données personnelles (PII), ou pour les agents résumés/digest qui se modifient eux-mêmes. La plupart des agents de modération n'ont **pas** besoin de cet outil - mark-spam, warn et ban couvrent le cycle de vie typique.

### Approbations

**Envisagez fortement de le soumettre à une approbation**, surtout pendant que vous établissez la confiance dans l'agent. Un agent qui réécrit les mots d'un utilisateur est le type d'action qu'une communauté remarquera et à laquelle elle réagira, et il est plus difficile de « défaire » sur le plan de la réputation qu'une suppression.

### Voir aussi

- [Trigger : Commentaire modifié](#trigger-comment-edit) - le trigger déclenché lorsque le texte d'un commentaire change.
- [Workflow d'approbation](#approval-workflow) - comment soumettre l'outil à une revue humaine.