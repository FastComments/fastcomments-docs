Lorsqu'un déclencheur se déclenche pour un agent mais **n'entraîne pas** d'appel LLM, la plateforme enregistre un "drop" avec une raison. Les drops apparaissent dans la [page Analytics](#analytics-page) sous « Déclencheurs ignorés (ce mois) ».

### La liste complète des raisons de drop

| Raison | Que s'est-il passé |
|---|---|
| `agentDaily` | Le plafond budgétaire quotidien de l'agent a été atteint. |
| `agentMonthly` | Le plafond budgétaire mensuel de l'agent a été atteint. |
| `tenantDaily` | Le plafond budgétaire quotidien du locataire a été atteint. |
| `tenantMonthly` | Le plafond budgétaire mensuel du locataire a été atteint. |
| `qps` | La limite de débit par minute (fenêtre glissante de 60s) de l'agent a été atteinte. |
| `concurrency` | Le nombre maximum d'exécutions simultanées de l'agent était déjà saturé. |

### Ce qui n'est pas dans cette liste

Un déclencheur qui n'atteint jamais le chemin d'envoi n'est pas enregistré comme un « drop » avec une raison — il n'est tout simplement pas envoyé. Cela inclut :

- L'agent est **désactivé**.
- Le commentaire déclencheur ne correspond pas à la [portée URL/locale](#scope-url-locale) de l'agent.
- L'action déclenchante a été effectuée par le même agent (prévention des boucles).
- Le locataire a une facturation invalide.
- L'agent ne fait pas partie du plan du locataire.

Ce sont des omissions silencieuses, pas des drops. Ils n'apparaissent pas dans le graphique des drops sur la page Analytics.

### Lecture des drops dans Analytics

La [page Analytics](#analytics-page) affiche :

- **Déclencheurs ignorés (ce mois)** - décomptes regroupés par raison de drop.
- **Agents atteignant ou proches de leur plafond** - répartition par agent montrant quels agents poussent le plafond, avec un nombre de déclencheurs dropped dans la période en cours.

### Que faire lorsque vous voyez des drops

- **`agentDaily` / `agentMonthly`** - le plafond de l'agent est trop bas. Soit augmentez le plafond dans le formulaire d'édition soit réduisez la portée de l'agent (URL/locale, déclencheurs plus restreints).
- **`tenantDaily` / `tenantMonthly`** - le plafond au niveau du locataire est trop bas. Augmentez-le dans les paramètres de facturation du locataire, ou répartissez les dépenses sur moins d'agents.
- **`qps`** - le trafic atteint la limite de fenêtre glissante par minute. Souvent signe d'un fil viral qui propage des déclencheurs plus vite que l'agent ne peut les exécuter. Les champs `maxTriggersPerMinute` et `maxConcurrent` de l'agent limitent cela ; les augmenter augmente le débit mais augmente aussi le coût en cas de pic.
- **`concurrency`** - même cause racine que `qps` mais au niveau du nombre d'exécutions en cours. Augmentez `maxConcurrent` si vous avez besoin de plus de parallélisme.

### Drops vs erreurs

Un drop signifie « le déclencheur ne s'est jamais exécuté ». Une **erreur** signifie « le déclencheur s'est exécuté mais l'appel LLM ou la distribution d'outil a échoué ». Les erreurs sont suivies séparément sur la [page Historique d'exécution](#run-history) (statut `Error`).

### Les drops peuvent aussi arrêter les relectures

Les mêmes raisons de drop interrompent aussi les [exécutions de test / relectures](#test-runs-replays) en cours. La relecture s'arrête avec un statut Error et un message indiquant quel budget a été atteint (par exemple, le budget quotidien de l'agent).

### La prévention des boucles est volontairement silencieuse

Il n'y a pas de raison de drop pour « ce déclencheur provient d'un autre agent et a été ignoré pour empêcher une boucle ». Le journaliser encombrerait les analytics sans apporter d'information utile — par conception, la propagation d'agents ne doit jamais entraîner une explosion de déclencheurs. Si vous suspectez qu'une boucle est supprimée alors qu'elle ne devrait pas l'être, vérifiez les [journaux de commentaires](/guide-moderation.html#comment-logs) — le `botId` des commentaires rédigés par le bot est ce sur quoi la vérification de boucle se base.

---