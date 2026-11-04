Les Agents IA sont disponibles sur les offres **Flex** et **Pro**. L'offre Creator ne les inclut pas.

### Limites au niveau de l'offre

Chaque niveau d'offre définit :

- **Des plafonds de budget quotidiens et mensuels par défaut.** Vous pouvez les réduire par agent ; augmenter le plafond par compte nécessite une offre avec un plafond supérieur. Voir [Vue d'ensemble des budgets](#budgets-overview).

Les chiffres exacts sont affichés sur la [page de tarification](https://fastcomments.com/traffic-pricing) et sur la page de facturation de votre compte. Ils sont également affichés directement dans le formulaire d'édition de l'agent, afin que vous n'ayez jamais à quitter le formulaire pour trouver votre plafond.

FastComments Pro inclut 200 $/mois d'utilisation de l'IA. Flex est facturé au tarif de 20 $ par million de tokens pour tous les modèles (actuellement GLM 5.2 ou gpt-oss-120B).

### La facturation doit être valide

Les Agents IA ne s'exécutent que si le tenant dispose d'une **facturation valide enregistrée**. Si le moyen de paiement devient invalide, tous les agents sont mis en pause et la page Agents IA affiche une bannière demandant à la personne ayant le rôle **Billing Admin** de mettre à jour la facturation. Les agents reprennent automatiquement une fois la facturation rétablie - aucune reprise ni rattrapage des déclencheurs déclenchés pendant l'interruption.

C'est une condition préalable stricte : les dépenses de tokens sont facturées sur votre compte, la plateforme n'exécutera donc aucun appel LLM sans un moyen de paiement fonctionnel.

### Qui peut gérer les agents

Les pages d'administration des agents sont protégées par le rôle de tableau de bord **Customization Admin**. Les **Comment Moderator Admins** peuvent examiner et décider des approbations (voir [Workflow d'approbation](#approval-workflow)) mais ne peuvent pas créer ni modifier d'agents. Les **Billing Admins** reçoivent les [e-mails d'alerte de budget](#budget-alerts), qu'ils aient ou non accès aux agents.