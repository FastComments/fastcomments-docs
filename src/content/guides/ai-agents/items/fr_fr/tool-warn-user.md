L'outil Warn envoie un avertissement privé par DM à un utilisateur au sujet d'un commentaire spécifique, et en même temps enregistre l'avertissement dans la [mémoire d'agent](#agent-memory-system) partagée. Les deux écritures sont atomiques - l'utilisateur ne voit jamais un avertissement qui n'est pas également consigné.

### Why it exists

La politique d'escalade de la plateforme est **avertir d'abord, bannir seulement en cas de récidive**. L'outil Warn rend cette politique applicable : il donne à l'utilisateur une chance de se corriger, et l'enregistrement de l'avertissement est ce qu'un futur agent trouve lorsqu'il consulte la mémoire avant d'envisager un bannissement.

L'outil évite aussi les doublons : si l'agent a déjà émis un avertissement au même utilisateur concernant le même commentaire, un deuxième avertissement est sans effet. Ainsi, un LLM qui boucle ou se déclenche à nouveau sur le même commentaire ne peut pas inonder l'utilisateur de multiples avertissements.

### What goes in the warning

Un message court (limité à 1000 caractères) affiché à l'utilisateur en DM. Un bon avertissement est :

- **Specific** - "Personal attacks on named users are not allowed in this community" bat "your comment was flagged."
- **Short** - a few sentences max.
- **Actionable** - tell the user what to change. "Please edit your comment to remove the named user, or it will be removed."

Vous n'écrivez pas le message vous-même ; l'agent le fait, en se basant sur le [initial prompt](#personality-prompt) et les [community guidelines](#community-guidelines). Votre travail consiste à rédiger un prompt qui produit de bons avertissements.

### When to allow it

Pour tout agent de type modération. Le modèle Moderator l'active par défaut.

### Approvals

Moins souvent soumis à approbation que [Ban user](#tool-ban-user). Il est utile de le soumettre à approbation pendant les premières semaines de la vie d'un agent afin de repérer les mauvais avertissements avant leur envoi, mais la plupart des opérateurs lèvent cette approbation une fois que l'agent produit des résultats fiables.

### See also

- [Ban user](#tool-ban-user) - the next step up in escalation.
- [Agent Memory System](#agent-memory-system) - where warning records live.