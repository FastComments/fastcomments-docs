Chaque agent s'exécute sur l'un des deux modèles LLM, choisi dans la section **Model** du formulaire d'édition.

### Les deux options

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - le défaut. Qualité de raisonnement supérieure, un peu plus lent par appel. Recommandé pour les agents de type modération (`Moderator` template, tout ce qui appelle `ban_user` ou `mark_comment_spam`) lorsque le coût d'une mauvaise action est élevé.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - plus rapide par appel, latence plus faible. Recommandé pour les agents à fort volume et faible enjeu (accueillant, épingleur de fil) où vous voulez des réponses en quelques secondes et où les conséquences d'une action incorrecte sont mineures.

Les deux modèles prennent en charge l'appel de fonctions, s'exécutent via la même OpenAI-compatible API et partagent les mêmes schémas par outil - vous pouvez donc basculer un agent enregistré entre eux à tout moment sans autres modifications de configuration.

### Différences de coût

Les deux modèles ont des coûts par token différents. Les [plafonds de budget](#budgets-overview) de l'agent sont libellés dans la devise de votre compte, pas en tokens, donc un passage d'un modèle à l'autre change le nombre d'exécutions qui tiennent dans vos plafonds quotidiens et mensuels. La page [Historique des exécutions](#run-history) affiche le coût par exécution dans votre devise une fois l'exécution terminée - regarder quelques exécutions après un changement est le moyen le plus simple d'évaluer le nouveau taux de consommation.

### Jetons par exécution

L'utilisation des tokens de réponse du modèle est enregistrée à chaque déclenchement sous la forme **tokensUsed**. Elle est incluse dans les charges utiles de webhook `trigger.succeeded` et `trigger.failed` (voir [Charges utiles des webhooks](#webhook-payloads)) et affichée dans la [Vue détaillée de l'exécution](#run-detail-view). La quantité dépend de :

- Combien de [Contexte](#context-options) vous incluez - le contexte du fil de discussion, l'historique de l'utilisateur et les métadonnées de la page ajoutent tous des tokens.
- La longueur de votre [prompt initial](#personality-prompt) et de vos [directives communautaires](#community-guidelines).
- Combien d'outils l'agent appelle en une seule exécution (chaque appel d'outil et son résultat effectuent un aller-retour à travers le modèle).

**Max Tokens Per Trigger** (default 20,000) est la limite supérieure par exécution, définie par agent.

### Changer de modèle

Vous pouvez changer de modèle dans le formulaire d'édition à tout moment. L'historique des exécutions et les analyses existantes conservent leurs nombres de tokens et de coûts d'origine - ils sont enregistrés au moment de l'exécution. Le nouveau modèle ne s'applique qu'aux exécutions qui commencent après que vous ayez enregistré.

Il n'y a pas d'option « use whichever model is cheaper ». Le choix est explicite par agent.