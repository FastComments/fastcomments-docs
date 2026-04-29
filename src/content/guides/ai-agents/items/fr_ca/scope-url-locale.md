---
Par défaut, un agent s'exécute sur tout votre tenant - chaque page, chaque locale. Les sections **Portée** et **Locales** du formulaire d'édition vous permettent de restreindre cela.

### Restreindre à des pages spécifiques

Le champ **Restrict to specific pages** accepte un modèle d'URL par ligne, en url-pattern glob syntax. L'agent ne s'exécute que sur les commentaires dont l'URL de la page correspond à au moins un des modèles. Exemples :

- `/news/*` - any page under `/news`.
- `/forums/*` - any page under `/forums`.
- `/blog/2026/*` - any page under `/blog/2026`.
- (multiple lines together) - the agent runs if **any** pattern matches.

Maximum : 50 motifs par agent. Les modèles doivent être des url-pattern globs valides - le formulaire rejette les modèles mal formés avec une erreur spécifique.

Quand le champ est **blank**, l'agent s'exécute sur chaque page du tenant.

Quand le champ est **non-blank**, l'agent échoue en mode fermé : tout déclencheur dont le commentaire n'a pas de `urlId` (p. ex. des événements au niveau du tenant sans contexte de page) est ignoré. C'est voulu - « scoped to /news/* » ne devrait pas retomber silencieusement sur « tout ».

### Restreindre à des locales spécifiques

Le sélecteur à double liste **Restrict to specific locales** accepte des identifiants de locale FastComments (`en_us`, `zh_cn`, `de_de`, etc.). L'agent ne s'exécute que sur les commentaires dont la locale détectée figure dans la liste sélectionnée.

La locale détectée provient du champ `locale` du commentaire, qui est définie par le widget de commentaire au moment de l'envoi en fonction de la locale de la page.

Quand **no locales** sont sélectionnées, l'agent s'exécute pour toutes les locales.

Quand **one or more locales** sont sélectionnées, l'agent échoue en mode fermé : les déclencheurs sans commentaire, ou les déclencheurs sur des commentaires sans champ `locale`, sont ignorés.

### Portée combinée

Les filtres d'URL et de locale sont combinés avec un ET. Un déclencheur ne lance l'agent que si **les deux** filtres l'autorisent.

Modèles utiles :
- `/news/*` URL pattern + `en_us` locale - seulement la section d'actualités en anglais.
- No URL filter + multiple locales - à l'échelle du tenant, mais uniquement pour les langues pour lesquelles l'invite de cet agent a été rédigée.

### Pourquoi restreindre la portée d'un agent

- **Coût.** La limitation de portée réduit le volume de déclencheurs que l'agent doit traiter, et donc diminue la consommation de tokens.
- **Exactitude.** Un prompt de résumé adapté aux articles techniques peut produire de mauvais résultats sur des pages produit. La limitation de portée est un outil plus précis que de demander au prompt de « ne pas traiter les pages non techniques » en anglais.
- **Comportement spécifique à la locale.** Un message de bienvenue qui s'exprime uniquement en allemand ne devrait s'exécuter que sur les commentaires en locale allemande. Combinez la portée `de_de` avec un ton en allemand dans l'[invite initiale](#personality-prompt).

### Ce que la limitation de portée ne fait *pas*

- Cela ne change pas le **nombre de slots d'agent** (voir [Forfaits et admissibilité](#plans-and-eligibility)) - un agent limité occupe toujours un slot.
- Cela ne change pas les [Plafonds budgétaires](#budgets-overview) - les limites journalières et mensuelles par agent s'appliquent à tous les déclencheurs correspondants.
- Cela ne restreint pas rétroactivement les exécutions passées - l'historique d'exécution affiche tout ce que l'agent a fait, même si vous resserrez la portée par la suite.

---