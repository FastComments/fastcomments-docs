Par défaut, un agent s'exécute sur l'ensemble de votre tenant - chaque page, chaque locale. Les sections **Scope** et **Locales** du formulaire d'édition vous permettent de restreindre ce périmètre.

### Restreindre à des pages spécifiques

Le champ **Restrict to specific pages** accepte un motif d'URL par ligne, en syntaxe glob url-pattern. L'agent ne s'exécute que sur les commentaires dont l'URL de page correspond à au moins un des motifs. Exemples :

- `/news/*` - toute page sous `/news`.
- `/forums/*` - toute page sous `/forums`.
- `/blog/2026/*` - toute page sous `/blog/2026`.
- (plusieurs lignes ensemble) - l'agent s'exécute si **un des** motifs correspond.

Maximum : 50 motifs par agent. Les motifs doivent être des globs url-pattern valides - le formulaire rejette les motifs mal formés avec une erreur spécifique.

Lorsque le champ est **vide**, l'agent s'exécute sur toutes les pages du tenant.

Lorsque le champ est **non vide**, l'agent échoue de manière sûre : tout déclencheur dont le commentaire n'a pas de `urlId` (par exemple les événements au niveau du tenant sans contexte de page) est ignoré. C'est voulu - « restreint à /news/* » ne doit pas silencieusement retomber sur « tout ».

### Restreindre à des locales spécifiques

Le sélecteur à double liste **Restrict to specific locales** accepte les identifiants de locale FastComments (`en_us`, `zh_cn`, `de_de`, etc.). L'agent ne s'exécute que sur les commentaires dont la locale détectée figure dans la liste sélectionnée.

La locale détectée provient du champ `locale` du commentaire, qui est défini par le widget de commentaires au moment de la publication en fonction de la locale de la page.

Lorsqu'**aucune locale** n'est sélectionnée, l'agent s'exécute sur toutes les locales.

Lorsqu'**une ou plusieurs locales** sont sélectionnées, l'agent échoue de manière sûre : les déclencheurs sans commentaire, ou les déclencheurs sur des commentaires sans champ `locale`, sont ignorés.

### Restriction combinée

Les filtres d'URL et de locale sont combinés avec un ET logique. Un déclencheur ne lance l'agent que si **les deux** filtres l'autorisent.

Motifs utiles :
- Motif d'URL `/news/*` + locale `en_us` - uniquement la section d'actualités en anglais.
- Pas de filtre d'URL + plusieurs locales - à l'échelle du tenant, mais uniquement pour les langues pour lesquelles le prompt de cet agent a été rédigé.

### Pourquoi restreindre le périmètre d'un agent

- **Coût.** La restriction réduit le volume de déclencheurs que l'agent doit traiter, et donc la dépense en tokens.
- **Exactitude.** Un prompt de résumé conçu pour des articles techniques peut produire un résultat médiocre sur des pages produits. La restriction est un outil plus précis que de demander au prompt de « ignorer les pages non techniques » en anglais.
- **Comportement spécifique à une locale.** Un message de bienvenue qui n'écrit qu'en allemand ne devrait s'exécuter que sur les commentaires de locale allemande. Combinez la restriction de locale `de_de` avec un ton en langue allemande dans le [prompt initial](#personality-prompt).

### Ce que la restriction ne fait *pas*

- Elle ne modifie pas le **nombre d'emplacements d'agents** (voir [Plans et éligibilité](#plans-and-eligibility)) - un agent restreint occupe toujours un emplacement.
- Elle ne modifie pas les [plafonds budgétaires](#budgets-overview) - les plafonds quotidiens et mensuels par agent s'appliquent à tous les déclencheurs correspondants.
- Elle ne restreint pas rétroactivement les exécutions passées - l'historique d'exécution montre tout ce que l'agent a fait, même si vous le restreignez davantage ensuite.