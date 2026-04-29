Chaque agent s'exécute avec l'un des deux modèles LLM, choisi dans la section **Modèle** du formulaire d'édition.

### Les deux options

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - par défaut. Qualité de raisonnement supérieure, un peu plus lent par appel. Recommandé pour les agents de type modération (template `Moderator`, tout ce qui appelle `ban_user` ou `mark_comment_spam`) lorsque le coût d'un appel erroné est élevé.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - plus rapide par appel, latence plus faible. Recommandé pour les agents à fort volume et faible enjeu (accueillant, épingleur de fil) lorsque vous souhaitez des réponses en quelques secondes et que les conséquences d'un appel raté sont mineures.

Les deux modèles prennent en charge le function calling, tous deux s'exécutent via la même API compatible OpenAI, et partagent les mêmes schémas par outil - vous pouvez donc basculer un agent sauvegardé entre eux à tout moment sans autres modifications de configuration.

### Différences de coût

Les deux modèles ont des coûts par token différents. Les [plafonds de budget](#budgets-overview) de l'agent sont libellés dans la monnaie de votre compte, pas en tokens, donc un passage d'un modèle à l'autre change le nombre d'exécutions qui tiennent dans vos plafonds journaliers et mensuels. L'[Historique des exécutions](#run-history) affiche le coût par exécution dans votre monnaie une fois qu'une exécution est terminée - surveiller quelques exécutions après un changement est le moyen le plus simple d'évaluer le nouveau rythme de consommation.

### Tokens par exécution

L'utilisation des tokens de la réponse du modèle est enregistrée à chaque déclenchement sous **tokensUsed**. Elle est incluse dans les payloads webhook `trigger.succeeded` et `trigger.failed` (voir [Charges utiles des webhooks](#webhook-payloads)) et affichée dans [Vue détaillée de l'exécution](#run-detail-view). La quantité dépend de :

- Combien de [Contexte](#context-options) vous incluez - le contexte du fil, l'historique utilisateur et les métadonnées de la page ajoutent tous des tokens.
- La longueur de votre [Prompt initial](#personality-prompt) et de vos [Consignes communautaires](#community-guidelines).
- Combien d'outils l'agent appelle lors d'une même exécution (chaque appel d'outil et son résultat font un aller-retour via le modèle).

**Nombre maximal de tokens par déclenchement** (par défaut 20 000) est la limite supérieure par exécution, défini par agent.

### Changement de modèle

Vous pouvez changer de modèle dans le formulaire d'édition à tout moment. L'historique des exécutions et les analyses existantes conservent leurs nombres de tokens et de coûts d'origine - ils sont enregistrés au moment de l'exécution. Le nouveau modèle ne s'applique qu'aux exécutions qui démarrent après votre sauvegarde.

Il n'existe pas d'option « utiliser le modèle le moins cher ». Le choix est explicite par agent.