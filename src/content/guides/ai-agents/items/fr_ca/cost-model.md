---
Le coût des agents est **basé sur les jetons**. Chaque appel LLM renvoie un nombre de jetons, la plateforme le convertit en cents USD en utilisant le tarif par jeton du modèle, et ces cents sont débités des budgets de l'agent et du locataire.

### Ce qui est facturé

- **Tous les appels LLM**, y compris l'appel qui ne produit aucune action d'outil ("l'agent a décidé de ne rien faire"). L'inférence est payée même lorsqu'aucune action n'en résulte.
- **Appels en mode exécution à sec**. L'exécution à sec signifie "ne pas agir, mais appeler quand même le LLM" — l'appel LLM coûte le même montant. Voir [Mode d'exécution à sec](#dry-run-mode).
- **Appels de relecture**. Les relectures sont des exécutions à sec contre des commentaires historiques. Elles consomment des jetons. Voir [Exécutions de test (Replays)](#test-runs-replays).

### Ce qui n'est pas facturé

- **Déclencheurs qui n'entraînent jamais d'appel LLM.** Les cas éliminés avant l'appel LLM (budget dépassé, limitation de débit, inadéquation de périmètre, facturation invalide, prévention de boucles) coûtent zéro jeton. Voir [Raisons d'abandon](#drop-reasons).
- **Lancement d'outils.** Appeler `pin_comment` ou tout autre outil ne coûte pas de jetons en soi — seul le aller-retour LLM en coûte.
- **`search_memory`.** Il est en lecture seule et ne génère pas son propre aller-retour LLM.

### Coût par exécution

Une seule exécution d'agent peut appeler le LLM plusieurs fois — chaque résultat d'appel d'outil est réinjecté dans le modèle afin qu'il puisse appeler un autre outil ou terminer. Ainsi, `tokensUsed` pour une exécution est la somme de tous les aller-retour LLM de cette exécution.

Les principaux éléments contribuant au coût en jetons par exécution :

- **Longs [prompts initiaux](#personality-prompt) et [directives communautaires](#community-guidelines)** — ils sont inclus à chaque exécution.
- **[Options de contexte](#context-options)** — contexte du fil, historique de l'utilisateur, métadonnées de la page. Chacun ajoute des jetons.
- **Le texte du commentaire lui-même** — les longs commentaires coûtent plus.
- **Appels multiples d'outils dans une même exécution** — le message de résultat de chaque outil est renvoyé au modèle.
- **Lectures de la mémoire** — `search_memory` renvoie jusqu'à 25 enregistrements (limité à 8000 caractères de contenu total). La plupart de ces octets sont intégrés dans le prompt suivant.

**Nombre maximal de jetons par déclenchement** (par défaut 20 000) limite la taille de la **réponse** par appel LLM. Cela ne limite pas la taille de l'entrée.

### Conversion des jetons en cents

La plateforme applique un seul tarif par paquet par locataire (`flexLLMCostCents` par `flexLLMUnit` jetons). Le coût par jeton est défini au niveau du forfait, pas par modèle — les deux modèles disponibles ([GLM 5.1 et GPT-OSS Turbo](#choosing-a-model)) sont facturés au même tarif pour un même forfait. La [Vue des détails d'exécution](#run-detail-view) affiche le coût par exécution dans votre devise une fois l'exécution terminée.

### Où le coût est enregistré

Chaque exécution enregistre son nombre brut de jetons et son coût par exécution. Les totaux quotidiens et mensuels sont regroupés sur la [page Analyses](#analytics-page).

### Comment lire le coût

- **Coût par exécution** : [Vue des détails d'exécution](#run-detail-view) -> champ `Cost`.
- **Agrégat quotidien / mensuel** : [page Analyses](#analytics-page) -> Utilisation du budget et graphiques du coût quotidien.
- **Coût par action** : également visible sur [Vue des détails d'exécution](#run-detail-view), utile pour l'ajustement lorsqu'une boucle d'outils d'un agent est anormalement longue.

### Voir aussi

- [Choisir un modèle](#choosing-a-model) - le levier le plus important pour les coûts.
- [Options de contexte](#context-options) - d'où provient le coût additionnel.
- [Aperçu des budgets](#budgets-overview) - des plafonds stricts qui empêchent les dépassements de coûts.

---