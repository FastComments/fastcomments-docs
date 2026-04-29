Par défaut, un agent s'exécute sur l'ensemble de votre tenant — chaque page, chaque locale. Les sections **Scope** et **Locales** du formulaire d'édition vous permettent de restreindre cela.

### Restrict to specific pages

Le champ **Restrict to specific pages** accepte un motif d'URL par ligne, en url-pattern glob syntax. L'agent ne s'exécute que sur les commentaires dont l'URL de page correspond à au moins un des motifs. Exemples :

- `/news/*` - toute page sous `/news`.
- `/forums/*` - toute page sous `/forums`.
- `/blog/2026/*` - toute page sous `/blog/2026`.
- (plusieurs lignes ensemble) - l'agent s'exécute si **au moins** un motif correspond.

Maximum : 50 motifs par agent. Les motifs doivent être des url-pattern globs valides — le formulaire rejette les motifs mal formés avec une erreur spécifique.

Lorsque le champ est **vide**, l'agent s'exécute sur chaque page du tenant.

Lorsque le champ est **non vide**, l'agent échoue fermé : tout trigger dont le commentaire n'a pas de `urlId` (par ex. des événements au niveau du tenant sans contexte de page) est ignoré. C'est volontaire — "scoped to /news/*" ne devrait pas silencieusement retomber sur "everything".

### Restrict to specific locales

Le sélecteur à double liste **Restrict to specific locales** accepte des IDs de locale FastComments (`en_us`, `zh_cn`, `de_de`, etc.). L'agent ne s'exécute que sur les commentaires dont la locale détectée est dans la liste sélectionnée.

La locale détectée provient du champ `locale` du commentaire, qui est défini par le widget de commentaire au moment de la publication en fonction de la locale de la page.

Quand **aucune locale** n'est sélectionnée, l'agent s'exécute pour toutes les locales.

Quand **une ou plusieurs locales** sont sélectionnées, l'agent échoue fermé : les triggers sans commentaire, ou les triggers sur des commentaires sans champ `locale`, sont ignorés.

### Combined scoping

Les filtres d'URL et de locale sont combinés avec un ET logique. Un trigger n'exécute l'agent que si **les deux** filtres l'autorisent.

Exemples utiles :
- `/news/*` URL pattern + `en_us` locale — uniquement la section actualités en anglais.
- Pas de filtre d'URL + plusieurs locales — à l'échelle du tenant, mais seulement pour les langues pour lesquelles l'invite de cet agent a été rédigée.

### Why scope an agent

- **Cost.** La restriction réduit le volume de triggers que l'agent doit traiter, et donc la consommation de tokens.
- **Correctness.** Une invite de résumé réglée pour des articles techniques peut produire de mauvais résultats sur des pages produit. La restriction est un outil plus précis que de demander à l'invite de "ne pas traiter les pages non techniques" en anglais.
- **Locale-specific behavior.** Un message de bienvenue qui ne doit écrire qu'en allemand ne devrait s'exécuter que sur les commentaires en locale allemande. Combinez la portée `de_de` avec un ton en allemand dans l'[initial prompt](#personality-prompt).

### What scoping does *not* do

- Cela ne change pas le **agent slot count** (voir [Plans and Eligibility](#plans-and-eligibility)) — un agent restreint occupe toujours un slot.
- Cela ne change pas les [Budget caps](#budgets-overview) — les plafonds journaliers et mensuels par agent s'appliquent à l'ensemble des triggers correspondants.
- Cela ne remet pas en périmètre rétroactivement les exécutions passées — l'historique d'exécution affiche tout ce que l'agent a fait, même si vous restreignez sa portée ensuite.

---