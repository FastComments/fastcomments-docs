Lorsqu'un déclencheur s'active pour un agent mais **n'aboutit pas** à un appel LLM, la plateforme enregistre un « drop » avec une raison. Les drops apparaissent dans la [page Analytics](#analytics-page) sous « Déclencheurs ignorés (ce mois-ci) ».

### Liste complète des raisons de drop

| Raison | Ce qui s'est passé |
|---|---|
| `agentDaily` | Le plafond budgétaire journalier de l'agent a été atteint. |
| `agentMonthly` | Le plafond budgétaire mensuel de l'agent a été atteint. |
| `tenantDaily` | Le plafond budgétaire journalier du tenant a été atteint. |
| `tenantMonthly` | Le plafond budgétaire mensuel du tenant a été atteint. |
| `qps` | Le quota par minute de l'agent (fenêtre glissante de 60 s) a été atteint. |
| `concurrency` | Le nombre maximal d'exécutions concurrentes de l'agent était déjà saturé. |

### Ce qui ne figure pas dans cette liste

Un déclencheur qui n'atteint jamais le chemin de distribution n'est pas « dropped » avec une raison — il n'est tout simplement pas distribué. Cela inclut :

- L'agent est **désactivé**.
- Le commentaire déclencheur ne correspond pas à la [portée URL/locale](#scope-url-locale) de l'agent.
- L'action déclenchante a été effectuée par le même agent (prévention de boucle).
- Le tenant a une facturation invalide.
- L'agent ne fait pas partie du plan du tenant.

Ce sont des omissions silencieuses, pas des drops. Elles n'apparaissent pas dans le graphique des drops sur la page Analytics.

### Lecture des drops sur la page Analytics

La [page Analytics](#analytics-page) affiche :

- **Déclencheurs ignorés (ce mois-ci)** — nombre d'éléments regroupés par raison de drop.
- **Agents atteignant ou proches de leur plafond** — répartition par agent indiquant quels agents poussent le plafond, avec un nombre de déclencheurs dropped pendant la période en cours.

### Que faire lorsque vous voyez des drops

- **`agentDaily` / `agentMonthly`** — le plafond propre à l'agent est trop bas. Soit augmentez le plafond dans le formulaire d'édition, soit réduisez la portée de l'agent (URL/locale, déclencheurs plus restreints).
- **`tenantDaily` / `tenantMonthly`** — le plafond au niveau du tenant est trop bas. Augmentez-le dans les paramètres de facturation du tenant, ou répartissez les dépenses sur moins d'agents.
- **`qps`** — le trafic atteint la limite par minute de la fenêtre glissante. Souvent signe d'un fil viral qui propage des déclencheurs plus vite que l'agent ne peut les exécuter. Les champs `maxTriggersPerMinute` et `maxConcurrent` de l'agent limitent cela ; les augmenter accroît le débit mais augmente aussi le coût des pointes.
- **`concurrency`** — même cause racine que `qps` mais sur le nombre d'exécutions en vol. Augmentez `maxConcurrent` si vous avez besoin de plus de parallélisme.

### Drops vs erreurs

Un drop signifie « le déclencheur ne s'est jamais exécuté ». Une **erreur** signifie « le déclencheur s'est exécuté mais l'appel LLM ou le dispatch d'outil a échoué ». Les erreurs sont suivies séparément sur la page [Historique d'exécution](#run-history) (statut `Error`).

### Les drops peuvent aussi arrêter les replays

Les mêmes raisons de drop stoppent les [exécutions de test / replays](#test-runs-replays) en cours. Le replay s'arrête avec un statut Error et un message indiquant quel budget a été atteint (par exemple, le budget journalier de l'agent).

### La prévention des boucles est volontairement silencieuse

Il n'existe pas de raison de drop pour « ce déclencheur provient d'un autre agent et a été ignoré pour prévenir une boucle ». Le consigner surchargerait les analytics sans signal utile — par conception, la propagation d'agents ne doit jamais provoquer une explosion de déclencheurs. Si vous soupçonnez qu'une boucle est supprimée alors qu'elle ne devrait pas l'être, vérifiez les [journaux de commentaires](/guide-moderation.html#comment-logs) — le `botId` des commentaires générés par un bot est ce sur quoi la vérification de boucle se base.

---