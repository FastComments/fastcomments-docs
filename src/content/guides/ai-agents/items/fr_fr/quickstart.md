Voici le chemin en cinq minutes pour passer de « nous avons des agents IA » à « un agent répond au trafic en direct, sous approbations ». Si vous préférez la version détaillée, chaque étape renvoie vers la page qui la couvre en profondeur.

### 1. Ouvrez la page AI Agents

Rendez-vous sur [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) dans votre compte. La première fois que vous arrivez ici, vous verrez soit :

- Un état vide avec un bouton **Browse templates** et **Start from scratch** (vous avez des agents disponibles à créer), ou
- Une page d'upsell si votre plan n'inclut pas les agents - voir [Plans and Eligibility](#plans-and-eligibility).

### 2. Choisissez un modèle de départ

Cliquez sur **Browse templates**. Choisissez l'un des suivants :

- [Moderator](#template-moderator) - examine les commentaires signalés ou nouveaux, avertit les nouveaux venus, n'escalade vers un bannissement qu'après un avertissement.
- [Welcome Greeter](#template-welcome-greeter) - répond aux commentateurs de première visite.
- [Top Comment Pinner](#template-top-comment-pinner) - épingle les commentaires substantiels une fois qu'ils dépassent un seuil de votes.
- [Thread Summarizer](#template-thread-summarizer) - publie un résumé neutre sur les longs fils de discussion.

Chaque modèle aboutit sur un formulaire d'édition pré-rempli avec **Status: Dry Run** déjà sélectionné.

### 3. Vérifiez et enregistrez

Sur le formulaire d'édition, faites au minimum :

- **Internal name.** Un identifiant court utilisé dans les tableaux de bord d'administration.
- **Display name.** Ce qui apparaît publiquement lorsque l'agent publie un commentaire.
- **Initial prompt.** Modifiez le prompt du modèle pour correspondre à votre ton et à vos règles spécifiques.
- **Approvals.** Cochez les actions qui devraient nécessiter une revue humaine avant de prendre effet. Nous recommandons au moins `ban_user` pour tout agent de type modération. Voir [Approval Workflow](#approval-workflow).

Cliquez sur **Save agent**.

### 4. Observez-le en dry-run

L'agent est maintenant actif en **Dry Run**. Il recevra ses déclencheurs, appellera le modèle et enregistrera des actions sur la page [Run History](#run-history) - avec le badge **Dry Run** sur chaque ligne - mais il ne prendra pas d'actions réelles. Consultez quelques détails d'exécution (voir [Run Detail View](#run-detail-view)) et examinez :

- Les actions choisies par l'agent.
- La justification et la confiance de chaque action.
- La transcription LLM complète.

Si l'agent prend des décisions avec lesquelles vous n'êtes pas d'accord, modifiez le prompt initial ou cochez davantage d'approbations.

### 5. Lancez un test sur des commentaires passés

Depuis la page de liste des agents, cliquez sur **Test run** sur la ligne de l'agent. Le formulaire comporte une seule entrée numérique **Days** (1 à 90). La taille de l'échantillon et la limite stricte de commentaires évalués sont affichées à titre informatif - elles sont calculées côté serveur, non définies par l'utilisateur. La relecture s'effectue sur les commentaires historiques sans prendre d'actions réelles et rapporte ce que l'agent **aurait** fait par rapport à ce qui s'est réellement passé (le commentaire a-t-il été approuvé ensuite, marqué comme spam, supprimé, etc.). Voir [Test Runs (Replays)](#test-runs-replays).

### 6. Passez en Enabled

Lorsque vous êtes satisfait du dry-run et du résultat de la relecture, modifiez l'agent et changez **Status** en **Enabled**. À partir de là, les actions réelles prennent effet. La page Run History affiche désormais les exécutions en direct sans le badge dry-run, et toute action que vous avez marquée pour approbation apparaît dans la [boîte de réception des approbations](#approval-workflow).

### Et ensuite

- Définissez des [Budgets](#budgets-overview) et des [Budget Alerts](#budget-alerts).
- Configurez des [Webhooks](#webhooks-overview) si vous souhaitez que des systèmes externes réagissent aux événements des agents.
- Ajoutez des [Community Guidelines](#community-guidelines) pour garder les décisions des agents alignées avec votre politique écrite.