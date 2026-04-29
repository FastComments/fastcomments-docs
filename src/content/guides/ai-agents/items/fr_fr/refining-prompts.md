**Affiner l'invite** est le flux de travail pour modifier l'[invite initiale](#personality-prompt) d'un agent en réponse à des décisions spécifiques avec lesquelles vous êtes en désaccord. Il se lance depuis la [boîte de réception des approbations](#approval-workflow).

### Quand l'utiliser

Lorsque vous vous retrouvez à rejeter encore et encore le même type d'approbation — « l'agent continue de vouloir bannir des personnes pour l'utilisation d'un langage fort sans cible » — l'invite de l'agent est le levier pour corriger cela. Affiner l'invite est une méthode guidée pour :

1. Choisir une approbation précise qui représente la mauvaise décision.
2. Modifier l'invite avec le contexte complet de ce que l'agent a fait et pourquoi.
3. Enregistrer la nouvelle invite pour l'agent.

Le résultat est un agent qui, dorénavant, serait peu susceptible de prendre la même décision.

### Lancement du flux

Depuis la boîte de réception des approbations à `/auth/my-account/ai-agent-approvals` :

1. Ouvrez une approbation **rejetée**. La route rejette fermement tout sauf REJECTED - pending et execution-failed approvals ne sont pas éligibles.
2. Cliquez sur **Affiner l'invite**.

Vous arrivez sur l'interface d'affinage de l'invite à `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Ce que la page affiche

- **L'approbation** - l'agent's `toolName` et `justification` pour la décision rejetée (la transcription complète du LLM n'est pas affichée ici).
- **L'invite actuelle** - l'[invite initiale](#personality-prompt) enregistrée de l'agent.
- **Un champ de commentaires** - vous saisissez des **commentaires** décrivant ce qui doit changer (jusqu'à 2000 caractères). Le LLM génère ensuite l'invite proposée à partir de vos commentaires.
- **Diff inline unifié** - un seul diff inline entre l'invite actuelle et l'invite proposée (rouge pour supprimé, vert pour ajouté).

Le contexte de l'approbation reste épinglé en haut afin que vous puissiez continuer à vous référer au « cas que je corrige » pendant l'édition.

### Enregistrer

L'enregistrement met à jour le champ `initialPrompt` de l'agent. Les exécutions passées (et les approbations passées) ne sont pas relancées rétroactivement - la nouvelle invite n'affecte que les déclencheurs futurs. Si vous voulez vérifier que la nouvelle invite corrige le problème, lancez un [test run / replay](#test-runs-replays) sur les 7 derniers jours et vérifiez si la nouvelle invite produirait encore l'approbation rejetée.

### Ce que le flux ne fait pas

- Il ne modifie pas les **lignes directrices communautaires** - ce champ a son propre éditeur dans le formulaire principal de modification de l'agent.
- Il ne modifie pas **triggers**, **allowed tools**, ou **approval gating** - ceux-ci restent sur le formulaire principal d'édition.
- Il ne versionne pas l'invite avec possibilité de retour en arrière. L'invite précédente n'est pas stockée dans une collection d'historique séparée. Si vous avez besoin de revenir en arrière, copiez l'invite actuelle dans votre propre système de suivi avant de modifier.

### Pourquoi associer l'affinage au replay

Modifier une invite sans tester le résultat relève de la foi. Le cycle recommandé :

1. Rejeter une approbation.
2. Affiner l'invite.
3. Lancer un [test run](#test-runs-replays) sur les 7 derniers jours.
4. Consultez l'onglet **Deltas**. La nouvelle invite a-t-elle déplacé la mauvaise décision de « would do » vers « would not do » ? A-t-elle accidentellement fait disparaître aussi des bonnes décisions ?
5. Itérer.

Trois ou quatre cycles d'affinage + replay suffisent généralement pour obtenir une invite stable pour un agent de modération.

### Alternative : édition directe

Vous n'êtes pas obligé d'utiliser Affiner l'invite - vous pouvez aussi simplement modifier l'agent via le formulaire principal d'édition. Le seul avantage d'Affiner l'invite est d'épingler un cas échouant spécifique afin que vous ne perdiez pas de vue ce que vous êtes en train de corriger.

---