**Affiner l'invite** est le flux de travail pour modifier l'[invite initiale](#personality-prompt) d'un agent en réponse à des décisions spécifiques avec lesquelles vous êtes en désaccord. Il est lancé depuis la [boîte de réception des approbations](#approval-workflow).

### Quand l'utiliser

Lorsque vous rejetez sans cesse le même type d'approbation — « l'agent veut sans cesse bannir des personnes pour avoir utilisé un langage fort sans cible » — l'invite de l'agent est le levier pour corriger cela. Affiner l'invite est une méthode guidée pour :

1. Choisir une approbation précise qui représente la mauvaise décision.
2. Modifier l'invite avec tout le contexte de ce que l'agent a fait et pourquoi.
3. Enregistrer la nouvelle invite pour l'agent.

Le résultat est un agent qui, dorénavant, aura peu de chances de prendre la même décision.

### Lancement du flux

Depuis la boîte de réception des approbations à `/auth/my-account/ai-agent-approvals` :

1. Ouvrez une approbation **rejetée**. La route rejette catégoriquement tout ce qui n'est pas REJECTED - les approbations en attente et celles avec échec d'exécution ne sont pas éligibles.
2. Cliquez sur **Affiner l'invite**.

Vous arrivez sur l'interface de refinement de l'invite à `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Ce que montre la page

- **L'approbation** - le `toolName` de l'agent et la `justification` de la décision rejetée (la transcription complète du LLM n'est pas affichée ici).
- **L'invite actuelle** - l'[invite initiale](#personality-prompt) enregistrée de l'agent.
- **Un champ de rétroaction** - vous tapez une **rétroaction** décrivant ce qui devrait changer (jusqu'à 2000 caractères). Le LLM génère alors la nouvelle invite proposée à partir de votre rétroaction.
- **Un diff en ligne unifié** - un seul diff en ligne entre l'invite actuelle et l'invite proposée (rouge pour le supprimé, vert pour l'ajouté).

Le contexte de l'approbation reste épinglé en haut afin que vous puissiez continuer à vous référer au « cas que je corrige » pendant la modification.

### Enregistrer

L'enregistrement met à jour le champ `initialPrompt` de l'agent. Les exécutions passées (et les approbations passées) ne sont pas relancées rétroactivement - la nouvelle invite n'affecte que les déclencheurs futurs. Si vous voulez vérifier que la nouvelle invite résout le problème, effectuez une [exécution de test / rejouer](#test-runs-replays) sur les 7 derniers jours et vérifiez si la nouvelle invite produirait toujours l'approbation rejetée.

### Ce que le flux ne fait pas

- Il ne modifie pas les **lignes directrices communautaires** - ce champ possède son propre éditeur dans le formulaire principal de modification de l'agent.
- Il ne modifie pas les **déclencheurs**, les **outils autorisés**, ni le **balayage d'approbation** - ceux-ci restent dans le formulaire principal d'édition.
- Il ne versionne pas l'invite avec possibilité de retour en arrière. L'invite précédente n'est pas stockée dans une collection d'historique séparée. Si vous devez revenir en arrière, copiez l'invite actuelle dans votre propre système de suivi avant de modifier.

### Pourquoi associer affiner et rejouer

Modifier une invite sans tester le résultat, c'est agir par foi. Le cycle recommandé :

1. Rejeter une approbation.
2. Affiner l'invite.
3. Exécuter une [exécution de test](#test-runs-replays) sur les 7 derniers jours.
4. Regarder l'onglet **Deltas**. La nouvelle invite a-t-elle déplacé la mauvaise décision de « ferait » à « ne ferait pas » ? A-t-elle accidentellement déplacé aussi des bonnes décisions hors de la catégorie ?
5. Itérer.

Trois ou quatre cycles d'affinage + rejouer suffisent généralement pour obtenir une invite stable pour un agent de modération.

### Alternative : modification directe

Vous n'êtes pas obligé d'utiliser Affiner l'invite — vous pouvez aussi simplement modifier l'agent dans le formulaire principal d'édition. L'avantage d'Affiner l'invite est qu'il épingle un cas défaillant précis afin que vous ne perdiez pas de vue ce que vous êtes en train de corriger.