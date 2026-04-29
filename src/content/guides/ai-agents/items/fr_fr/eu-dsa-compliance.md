FastComments applique l'article 17 du Digital Services Act de l'UE pour les locataires dans la région UE : **les suspensions d'utilisateurs entièrement automatisées ne sont pas autorisées**.

### Ce que cela signifie en pratique

Lorsque votre locataire est dans la région UE, sur le formulaire d'édition d'agent :

- La case à cocher **Approvals** pour `ban_user` est verrouillée en position activée et ne peut pas être décochée.
- L'étiquette indique : "EU DSA Article 17 : les suspensions d'utilisateurs nécessitent un examen humain. 'Bannir un utilisateur' est verrouillé en position activée et ne peut pas être entièrement automatisé dans la région UE."
- Une infobulle sur la colonne des approbations indique : "Verrouillé par l'article 17 du DSA de l'UE - les bannissements entièrement automatisés ne sont pas autorisés dans la région UE."

Quoi que vous configuriez d'autre, chaque appel `ban_user` provenant de n'importe quel agent sur un locataire dans la région UE est envoyé à la [boîte de réception des approbations](#approval-workflow) pour examen humain. Le bannissement n'a pas lieu tant qu'un humain ne l'approuve pas.

### Pourquoi ceci est appliqué au niveau de la plateforme, et non au niveau du prompt

Les system prompts peuvent être ignorés ou contournés par un modèle suffisamment malveillant. La conformité à l'article 17 est trop importante pour dépendre du bon comportement du modèle ; elle doit être une barrière serveur stricte que le répartiteur d'outils lui-même applique. C'est ce que nous faisons.

### Ce qui passe ou ne passe pas par approbation

- **`ban_user`** : toujours filtré dans l'UE. Y compris :
  - Bannissements visibles (`shadowBan: false`).
  - Bannissements invisibles (`shadowBan: true`).
  - Bannissements avec `deleteAllUsersComments: true`.
  - Bannissements avec `banIP: true`.
- Toutes les variantes de bannissement atterrissent dans la boîte de réception des approbations avec le raisonnement et la confiance de l'agent ; un humain approuve ou rejette.

Les autres outils d'agent (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) **ne** sont **pas** affectés par l'article 17. Vous pouvez toujours les automatiser. L'article 17 concerne spécifiquement les suspensions d'utilisateurs.

### Et pour les locataires hors UE

Le verrou ne s'applique pas en dehors de la région UE. Vous pouvez choisir de soumettre `ban_user` à une approbation de toute façon — nous le recommandons fortement durant les premières semaines de fonctionnement de tout agent de modération — mais ce n'est pas imposé.

### Bannissements invisibles

Les bannissements invisibles sont considérés comme des suspensions aux fins de l'article 17 (l'utilisateur peut publier mais son contenu est masqué). Ils sont soumis aux mêmes règles que les bannissements visibles.

### Détection de la région

La région est déterminée au niveau du processus par la variable d'environnement `REGION` sur le déploiement FastComments (lue par `isEURegion()` dans `models/constants.ts`). Il n'existe pas de champ de région par locataire - le verrou s'applique à chaque locataire sur une instance déployée dans l'UE. Si vous migrez vos données d'un déploiement hors UE vers un déploiement dans l'UE, le verrou prend effet pour tous les locataires de cette instance.

### Et si tous les réviseurs sont indisponibles

L'approbation restera dans la boîte de réception jusqu'à décision. Elle expire automatiquement 90 jours après sa création. Il n'existe pas de voie "aucun réviseur disponible, basculer vers une décision automatisée" — cela irait à l'encontre de l'objectif de l'article 17.

Si votre communauté a un volume si élevé que les bannissements dans l'UE ne peuvent pas être examinés dans un délai raisonnable, envisagez :

- D'ajouter davantage de réviseurs (voir [Approval Notifications](#approval-notifications)).
- De configurer l'agent pour utiliser de manière plus aggressive [`warn_user`](#tool-warn-user), puisque les avertissements ne sont pas soumis à l'article 17.
- De réduire l'appétence de l'agent pour le bannissement en resserrant les [directives communautaires](#community-guidelines) ou le [prompt initial](#personality-prompt).

### Voir aussi

- [Tool: ban_user](#tool-ban-user) pour ce que fait `ban_user` et les options destructrices derrière des opt-ins supplémentaires.
- [Approval Workflow](#approval-workflow) pour l'ensemble du cycle de vie d'approbation.