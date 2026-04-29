Les agents d'IA sont disponibles dans les forfaits **Flex** et **Pro**. Le forfait Creator ne les inclut pas.

### Limites au niveau du forfait

Chaque niveau de forfait définit :

- **Plafonds de budget quotidiens et mensuels par défaut.** Vous pouvez les diminuer par agent ; augmenter le plafond par compte nécessite un forfait offrant un plafond supérieur. Consultez [Aperçu des budgets](#budgets-overview).

Les chiffres exacts sont indiqués sur la [page de tarification](https://fastcomments.com/traffic-pricing) et sur la page de facturation de votre compte. Ils sont également affichés directement sur le formulaire de modification de l'agent, vous n'avez donc jamais à quitter le formulaire pour trouver votre plafond.

FastComments Pro inclut 200 $/mo d'utilisation d'IA. Flex est facturé au taux de 20 $ par million de tokens pour tous les modèles (actuellement soit GLM 5.1 soit gpt-oss-120B-turbo).

### La facturation doit être valide

Les agents d'IA ne fonctionnent que lorsque le locataire a une **facturation valide enregistrée**. Si la méthode de paiement devient invalide, tous les agents sont mis en pause et la page Agents d'IA affiche une bannière demandant à la personne ayant le rôle **Billing Admin** de mettre à jour la facturation. Les agents reprennent automatiquement une fois la facturation rétablie — il n'y a pas de relecture ni de rattrapage des déclencheurs qui se sont déclenchés pendant la panne.

Ceci est un prérequis strict : les dépenses en tokens sont facturées sur votre compte, donc la plateforme n'enverra aucun appel LLM sans une méthode de paiement opérationnelle.

### Qui peut gérer les agents

Les pages d'administration des agents sont accessibles uniquement aux détenteurs du rôle de tableau de bord **Customization Admin**. Les **Comment Moderator Admins** peuvent examiner et décider des approbations (voir [Flux d'approbation](#approval-workflow)) mais ne peuvent pas créer ni modifier des agents. Les **Billing Admins** reçoivent des [courriels d'alerte de budget](#budget-alerts) qu'ils aient ou non accès aux agents.