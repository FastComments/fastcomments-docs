Ceci est le parcours de cinq minutes allant de "nous avons des agents d'IA" à "un agent répond au trafic en direct, soumis à des approbations". Si vous voulez la version longue, chaque étape contient un lien vers la page qui la couvre en profondeur.

### 1. Ouvrir la page Agents d'IA

Allez à [Agents d'IA](https://fastcomments.com/auth/my-account/ai-agents) dans votre compte. La première fois que vous arrivez ici, vous verrez soit :

- Un état vide avec un bouton **Parcourir les modèles** et **Commencer de zéro** (vous avez des agents disponibles à créer), ou
- Une page d'incitation si votre forfait n'inclut pas les agents - voir [Forfaits et admissibilité](#plans-and-eligibility).

### 2. Choisir un modèle de départ

Cliquez sur **Parcourir les modèles**. Choisissez l'un des suivants :

- [Modérateur](#template-moderator) - examine les commentaires signalés ou nouveaux, avertit les primo-contrevenants, n'applique une interdiction qu'après un avertissement.
- [Accueillant](#template-welcome-greeter) - répond aux commentateurs pour la première fois.
- [Épingleur de meilleur commentaire](#template-top-comment-pinner) - épingle les commentaires substantiels une fois qu'ils dépassent un seuil de votes.
- [Résumeur de fil](#template-thread-summarizer) - publie un résumé neutre sur les longues discussions.

Chaque modèle ouvre un formulaire d'édition pré-rempli avec **Statut : Exécution de test** déjà sélectionné.

### 3. Réviser et enregistrer

Sur le formulaire d'édition, remplissez au minimum :

- **Nom interne.** Un identifiant court utilisé dans les tableaux de bord d'administration.
- **Nom d'affichage.** Ce qui apparaît publiquement lorsqu'un agent publie un commentaire.
- **Invite initiale.** Modifiez l'invite du modèle pour correspondre à votre ton et à vos règles spécifiques.
- **Approbations.** Cochez les actions qui devraient nécessiter une révision humaine avant d'entrer en vigueur. Nous recommandons au minimum `ban_user` pour tout agent de type modération. Voir [Flux d'approbation](#approval-workflow).

Cliquez sur **Enregistrer l'agent**.

### 4. Observer en exécution de test

L'agent est maintenant actif en **Exécution de test**. Il recevra ses déclencheurs, appellera le modèle et enregistrera les actions sur la page [Historique d'exécution](#run-history) - avec le badge **Exécution de test** sur chaque ligne - mais il n'exécute pas d'actions réelles. Consultez quelques détails d'exécution (voir [Vue détaillée d'exécution](#run-detail-view)) et examinez :

- Les actions choisies par l'agent.
- La justification et la confiance pour chaque action.
- la transcription complète du LLM.

Si l'agent prend des décisions avec lesquelles vous n'êtes pas d'accord, modifiez l'invite initiale ou cochez plus d'approbations.

### 5. Effectuer un test sur des commentaires passés

Depuis la page de la liste des agents, cliquez sur **Exécution de test** sur la ligne de l'agent. Le formulaire comporte un seul champ numérique **Jours** (1 à 90). La taille d'échantillon et le plafond dur sur les commentaires évalués sont affichés à titre informatif - ils sont calculés côté serveur, non définis par l'utilisateur. La relecture s'exécute contre des commentaires historiques sans effectuer d'actions réelles et rapporte ce que l'agent **aurait** fait par rapport à ce qui s'est réellement produit (le commentaire a-t-il ensuite été approuvé, marqué comme spam, supprimé, etc.). Voir [Exécutions de test (relectures)](#test-runs-replays).

### 6. Passer à Activé

Lorsque vous êtes satisfait de l'exécution de test et des résultats de la relecture, modifiez l'agent et changez **Statut** à **Activé**. À partir de ce moment, les actions réelles sont appliquées. La page Historique d'exécution affiche désormais des exécutions en direct sans le badge d'exécution de test, et toute action que vous avez marquée pour approbation apparaît dans la [boîte de réception des approbations](#approval-workflow).

### Prochaines étapes

- Définissez des [Budgets](#budgets-overview) et des [Alertes de budget](#budget-alerts).
- Configurez des [Webhooks](#webhooks-overview) si vous souhaitez que des systèmes externes réagissent aux événements des agents.
- Ajoutez des [Directives communautaires](#community-guidelines) pour maintenir les décisions des agents alignées sur votre politique écrite.