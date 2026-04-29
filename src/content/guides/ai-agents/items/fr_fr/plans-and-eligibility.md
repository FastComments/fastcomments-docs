Les AI Agents sont disponibles sur les plans **Flex** et **Pro**. Le plan Creator ne les inclut pas.

### Limites par plan

Chaque niveau de plan définit :

- **Plafonds de budget par défaut quotidiens et mensuels.** Vous pouvez réduire ces plafonds par agent ; augmenter le plafond par compte nécessite un plan avec un seuil supérieur. Voir [Aperçu des budgets](#budgets-overview).

Les chiffres exacts sont affichés sur la [page de tarification](https://fastcomments.com/traffic-pricing) et sur la page de facturation de votre compte. Ils sont également affichés dans le formulaire d'édition de l'agent, afin que vous n'ayez jamais à quitter le formulaire pour trouver votre plafond.

FastComments Pro inclut 200 $/mo d'utilisation d'IA. Flex est facturé au tarif de 20 $ par million de tokens pour tous les modèles (actuellement soit GLM 5.1, soit gpt-oss-120B-turbo).

### La facturation doit être valide

Les AI Agents ne fonctionnent que lorsque le locataire dispose d'une **facturation valide enregistrée**. Si le mode de paiement devient invalide, tous les agents sont mis en pause et la page AI Agents affiche une bannière demandant à la personne ayant le rôle **Billing Admin** de mettre à jour la facturation. Les agents reprennent automatiquement une fois la facturation rétablie — il n'y a ni rejouage ni rattrapage des déclencheurs qui se sont produits pendant la panne.

Ceci est une exigence stricte : les dépenses en tokens sont facturées sur votre compte, donc la plateforme n'engagera aucun appel LLM sans méthode de paiement fonctionnelle.

### Qui peut gérer les agents

Les pages d'administration des agents sont protégées par le rôle de tableau de bord **Customization Admin**. Les **Comment Moderator Admins** peuvent examiner et décider des approbations (voir [Flux d'approbation](#approval-workflow)) mais ne peuvent pas créer ni modifier des agents. Les **Billing Admins** reçoivent des [e-mails d'alerte de budget](#budget-alerts) qu'ils aient ou non accès aux agents.