FastComments applique l'article 17 du Digital Services Act de l'UE pour les locataires dans la région UE : **les suspensions d'utilisateurs entièrement automatisées ne sont pas autorisées**.

### Ce que cela signifie en pratique

Lorsque votre locataire se trouve dans la région UE, sur le formulaire d'édition de l'agent :

- La case à cocher **Approbations** pour `ban_user` est **cochée et verrouillée** et ne peut pas être décochée.
- Le libellé indique : "Article 17 du DSA de l'UE : les suspensions d'utilisateurs nécessitent un examen humain. 'Bannir un utilisateur' est cochée et ne peut pas être entièrement automatisée dans la région UE."
- Une info-bulle sur la colonne des approbations indique : "Verrouillé par l'article 17 du DSA de l'UE - les bannissements entièrement automatisés ne sont pas autorisés dans la région UE."

Quoi que vous configuriez d'autre, chaque appel `ban_user` émis par n'importe quel agent sur un locataire dans la région UE est envoyé à la [boîte de réception des approbations](#approval-workflow) pour examen humain. Le bannissement n'a lieu que lorsqu'un humain l'approuve.

### Pourquoi cela est appliqué au niveau de la plateforme et non au niveau du prompt

Les prompts système peuvent être ignorés ou contournés par un modèle suffisamment malveillant. La conformité à l'article 17 est trop importante pour être laissée au bon comportement du modèle ; il doit s'agir d'un verrou strict côté serveur que le répartiteur d'outils lui-même applique. C'est ce que nous faisons.

### Ce qui est et n'est pas soumis à approbation

- **`ban_user`** : toujours soumis à un verrouillage dans l'UE. Cela inclut :
  - Bannissements visibles (`shadowBan: false`).
  - Bannissements cachés (`shadowBan: true`).
  - Bannissements avec `deleteAllUsersComments: true`.
  - Bannissements avec `banIP: true`.
- Toutes les variantes de bannissement atterrissent dans la boîte de réception des approbations avec le raisonnement et le niveau de confiance de l'agent ; un humain approuve ou rejette.

Les autres outils d'agent (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) **ne** sont **pas** affectés par l'article 17. Vous pouvez toujours les automatiser. L'article 17 porte spécifiquement sur les suspensions d'utilisateurs.

### Et pour les locataires hors UE

Le verrou ne s'applique pas en dehors de la région UE. Vous pouvez choisir de soumettre `ban_user` à une approbation quand même — nous le recommandons fortement pendant les premières semaines de vie d'un agent de modération — mais ce n'est pas appliqué de force.

### Bannissements cachés

Les bannissements cachés sont considérés comme des suspensions aux fins de l'article 17 (l'utilisateur peut publier mais son contenu est masqué). Ils sont soumis aux mêmes verrous que les bannissements visibles.

### Détection de la région

La région est déterminée au niveau du processus par la variable d'environnement `REGION` sur le déploiement FastComments (lue par `isEURegion()` dans `models/constants.ts`). Il n'existe pas de champ de région par locataire : le verrou s'applique à chaque locataire sur une instance déployée dans l'UE. Si vous migrez vos données d'un déploiement hors UE vers un déploiement dans l'UE, le verrou prend effet pour tous les locataires de cette instance.

### Que faire si tous les réviseurs sont indisponibles

L'approbation restera dans la boîte de réception jusqu'à décision. Elle expire automatiquement 90 jours après sa création. Il n'existe pas de parcours "aucun réviseur disponible, basculer vers une décision automatisée" — cela irait à l'encontre de l'objectif de l'article 17.

Si votre communauté génère tellement de volume que les bannissements dans l'UE ne peuvent pas être examinés en temps raisonnable, envisagez :

- D'ajouter davantage de réviseurs (voir [Approval Notifications](#approval-notifications)).
- De configurer l'agent pour utiliser [`warn_user`](#tool-warn-user) de façon plus agressive, puisque les avertissements ne sont pas soumis à l'article 17.
- De réduire l'appétit de l'agent pour le bannissement en resserrant les [directives communautaires](#community-guidelines) ou le [prompt initial](#personality-prompt).

### Voir aussi

- [Tool: ban_user](#tool-ban-user) pour ce que fait `ban_user` et les options destructrices derrière des consentements supplémentaires.
- [Approval Workflow](#approval-workflow) pour le cycle de vie complet des approbations.