Le coût des agents est **basé sur les jetons**. Chaque appel LLM retourne un décompte de jetons, la plateforme convertit cela en cents USD en utilisant le tarif par jeton du modèle, et les cents sont débités des budgets de l'agent et du tenant.

### Ce qui est facturé

- **Tous les appels LLM**, y compris l'appel qui ne produit aucune action d'outil ("l'agent a décidé de ne rien faire"). L'inférence est payée même lorsqu'aucune action ne résulte.
- **Les appels en dry-run**. Le dry-run signifie "ne pas agir, mais appeler quand même le LLM" - l'appel LLM coûte le même montant. Voir [Mode Dry-Run](#dry-run-mode).
- **Les appels de replay**. Les replays sont des exécutions en dry-run contre des commentaires historiques. Ils coûtent des jetons. Voir [Test Runs (Replays)](#test-runs-replays).

### Ce qui n'est pas facturé

- **Les triggers qui ne produisent jamais d'appel LLM.** Les cas "dropped-before-LLM" (budget dépassé, limitation de débit, inadéquation de portée, facturation invalide, prévention de boucle) coûtent zéro jeton. Voir [Drop Reasons](#drop-reasons).
- **Le dispatch d'outils.** Appeler `pin_comment` ou tout autre outil ne coûte pas de jetons en soi - seul le aller-retour LLM est facturé.
- **`search_memory`.** Il est en lecture seule et ne génère pas son propre aller-retour LLM.

### Coût par exécution

Une seule exécution d'agent peut appeler le LLM plusieurs fois - chaque résultat d'appel d'outil est renvoyé au modèle afin qu'il puisse soit appeler un autre outil soit finir. Ainsi, `tokensUsed` sur une exécution est la somme de tous les aller-retours LLM dans cette exécution.

Les principaux contributeurs au coût par exécution :

- **Longs [prompts initiaux](#personality-prompt) et [directives communautaires](#community-guidelines)** - ils sont inclus à chaque exécution.
- **[Options de contexte](#context-options)** - contexte du fil, historique utilisateur, métadonnées de page. Chacun ajoute des jetons.
- **Le texte du commentaire lui-même** - les commentaires longs coûtent plus.
- **Multiples appels d'outils dans une même exécution** - le message de résultat de chaque outil est renvoyé au modèle.
- **Lectures de la mémoire** - `search_memory` retourne jusqu'à 25 enregistrements (limité à 8000 caractères de contenu total). La plupart de ces octets sont inclus dans le prompt suivant.

**Max Tokens Per Trigger** (default 20,000) plafonne la taille de la **réponse** par appel LLM. Elle ne plafonne pas la taille de l'entrée.

### Conversion jetons -> cents

La plateforme applique un seul tarif par package du locataire (`flexLLMCostCents` par `flexLLMUnit` jetons). Le coût par jeton est défini au niveau du package, pas par modèle - les deux modèles disponibles ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) sont facturés au même tarif pour un package donné. La [Run Detail View](#run-detail-view) affiche le coût par exécution dans votre devise une fois qu'une exécution est terminée.

### Où le coût est enregistré

Chaque exécution enregistre son nombre brut de jetons et le coût par exécution. Les totaux journaliers et mensuels sont consolidés dans la [page Analytics](#analytics-page).

### Comment lire le coût

- **Coût par exécution** : [Run Detail View](#run-detail-view) -> champ `Cost`.
- **Agrégat journalier / mensuel** : [page Analytics](#analytics-page) -> Utilisation du budget et graphiques du coût journalier.
- **Coût par action** : également sur la Run Detail View, utile pour l'optimisation lorsque la boucle d'outils d'un agent est anormalement longue.

### Voir aussi

- [Choisir un modèle](#choosing-a-model) - le levier le plus important sur le coût.
- [Options de contexte](#context-options) - d'où proviennent les coûts additionnels.
- [Aperçu des budgets](#budgets-overview) - plafonds stricts qui empêchent les coûts de s'emballer.