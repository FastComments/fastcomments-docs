---
Voici le parcours de cinq minutes allant de "nous disposons d'agents IA" à "un agent répond au trafic en direct, soumis à des validations". Si vous souhaitez la version longue, chaque étape renvoie à la page qui la couvre en profondeur.

### 1. Open the AI Agents page

Allez sur [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) dans votre compte. La première fois que vous arrivez ici, vous verrez soit :

- Un écran vide avec un bouton **Parcourir les modèles** et **Commencer à partir de zéro** (vous avez des agents disponibles à créer), soit
- Une page de promotion si votre offre n'inclut pas les agents - voir [Plans and Eligibility](#plans-and-eligibility).

### 2. Pick a starter template

Cliquez sur **Parcourir les modèles**. Choisissez l’un des éléments suivants :

- [Modérateur](#template-moderator) - examine les commentaires signalés ou nouveaux, prévient les primo-délinquants, n'applique une interdiction qu'après un avertissement.
- [Message d'accueil](#template-welcome-greeter) - répond aux commentateurs pour la première fois.
- [Épingleur de commentaires populaires](#template-top-comment-pinner) - épingle les commentaires substantiels une fois qu'ils dépassent un seuil de votes.
- [Résumé de fil](#template-thread-summarizer) - publie un résumé neutre sur les fils longs.

Chaque modèle ouvre un formulaire d'édition prérempli avec **Statut : Mode simulation** déjà sélectionné.

### 3. Review and save

Sur le formulaire d'édition, renseignez au minimum :

- **Nom interne.** Un identifiant court utilisé dans les tableaux de bord d'administration.
- **Nom affiché.** Ce qui apparaît publiquement lorsque l'agent publie un commentaire.
- **Invite initiale.** Modifiez l'invite du modèle pour qu'elle corresponde à votre ton et à vos règles spécifiques.
- **Approbations.** Cochez les actions qui doivent faire l'objet d'un examen humain avant d'entrer en vigueur. Nous recommandons au minimum `ban_user` pour tout agent de type modération. Voir [Approval Workflow](#approval-workflow).

Cliquez sur **Enregistrer l'agent**.

### 4. Watch it in dry-run

L'agent est maintenant actif en **Mode simulation**. Il recevra ses déclencheurs, appellera le modèle et enregistrera les actions sur la page [Run History](#run-history) - avec le badge **Mode simulation** sur chaque ligne - mais il n'exécute pas d'actions réelles. Consultez quelques détails d'exécution (voir [Run Detail View](#run-detail-view)) et regardez :

- Les actions choisies par l'agent.
- La justification et la confiance associées à chaque action.
- la transcription complète du LLM.

Si l'agent prend des décisions avec lesquelles vous n'êtes pas d'accord, modifiez l'invite initiale ou cochez davantage d'approbations.

### 5. Run a test against past comments

Depuis la page de la liste des agents, cliquez sur **Test run** sur la ligne de l'agent. Le formulaire comporte un seul champ numérique **Jours** (1 à 90). La taille de l'échantillon et le plafond strict sur le nombre de commentaires évalués sont affichés à titre informatif - ils sont calculés côté serveur, non configurables par l'utilisateur. La relecture s'exécute sur des commentaires historiques sans effectuer d'actions réelles et indique ce que l'agent **aurait** fait par rapport à ce qui s'est réellement passé (le commentaire a-t-il ensuite été approuvé, marqué comme spam, supprimé, etc.). Voir [Test Runs (Replays)](#test-runs-replays).

### 6. Flip to Enabled

Lorsque vous êtes satisfait du mode simulation et des résultats de la relecture, modifiez l'agent et changez le **Statut** en **Activé**. À partir de ce moment, les actions réelles seront appliquées. La page Run History affiche désormais des exécutions en direct sans le badge Mode simulation, et toute action que vous avez marquée pour approbation apparaît dans la [boîte de réception des approbations](#approval-workflow).

### What's next

- Définissez des [Budgets](#budgets-overview) et des [Alertes de budget](#budget-alerts).
- Configurez les [Webhooks](#webhooks-overview) si vous souhaitez que des systèmes externes réagissent aux événements des agents.
- Ajoutez des [Directives communautaires](#community-guidelines) pour aligner les décisions de l'agent sur votre politique écrite.

---