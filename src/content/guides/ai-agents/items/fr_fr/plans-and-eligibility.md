Les agents IA sont disponibles sur les plans **Flex** et **Pro**. Le plan Creator ne les inclut pas.

### Plan-level limits

Chaque niveau de plan définit :

- **Limites de budget quotidiennes et mensuelles par défaut.** Vous pouvez les réduire par agent ; augmenter la limite par compte nécessite un plan avec un plafond plus élevé. Voir [Budgets Overview](#budgets-overview).

Les chiffres exacts sont affichés sur la [pricing page](https://fastcomments.com/traffic-pricing) et sur la page de facturation de votre compte. Ils sont également affichés en ligne sur le formulaire d’édition de l’agent afin que vous n’ayez jamais à quitter le formulaire pour trouver votre limite.

FastComments Pro inclut 200 $/mo d’utilisation d’IA. Flex est facturé au tarif de 20 $ par million de jetons pour tous les modèles (actuellement soit GLM 5.2 soit gpt-oss-120B).

### Billing must be valid

Les agents IA ne fonctionnent que lorsque le locataire dispose d’une **facturation valide enregistrée**. Si le mode de paiement devient invalide, tous les agents sont mis en pause et la page des agents IA affiche une bannière dirigeant la personne qui possède le rôle **Billing Admin** à mettre à jour la facturation. Les agents reprennent automatiquement une fois la facturation rétablie – aucun relecture ou remplissage rétroactif des déclencheurs qui se sont activés pendant la panne.

C’est une condition préalable stricte : la dépense de jetons est facturée à votre compte, donc la plateforme n’enverra aucun appel LLM sans un mode de paiement fonctionnel.

### Who can manage agents

Les pages d’administration des agents sont réservées au rôle **Customization Admin** du tableau de bord. Les **Comment Moderator Admins** peuvent examiner et décider des approbations (voir [Approval Workflow](#approval-workflow)) mais ne peuvent pas créer ou modifier des agents. Les **Billing Admins** reçoivent les [budget alert emails](#budget-alerts) quel que soit leur accès aux agents.