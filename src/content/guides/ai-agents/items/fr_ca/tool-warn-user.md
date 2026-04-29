L'outil Warn envoie un avertissement privé par DM à un utilisateur au sujet d'un commentaire spécifique, et en même temps enregistre l'avertissement dans la [mémoire d'agent partagée](#agent-memory-system). Les deux écritures sont atomiques - l'utilisateur ne voit jamais un avertissement qui n'est pas également consigné.

### Why it exists

La politique d'escalade de la plateforme est **avertir d'abord, bannir seulement si l'utilisateur récidive**. L'outil Warn rend cette politique actionnable : il donne à l'utilisateur une chance de se corriger, et le dossier d'avertissement est ce qu'un futur agent trouve lorsqu'il recherche dans la mémoire avant d'envisager un bannissement.

L'outil déduplique aussi : si l'agent a déjà émis un avertissement au même utilisateur concernant le même commentaire, un deuxième avertissement n'a aucun effet. Ainsi, un LLM qui boucle ou se réactive sur le même commentaire ne peut pas inonder l'utilisateur de plusieurs avertissements.

### What goes in the warning

Un court message (limité à 1000 caractères) montré à l'utilisateur comme DM. Les avertissements efficaces sont :

- **Spécifique** - "Les attaques personnelles contre des utilisateurs nommés ne sont pas autorisées dans cette communauté" est mieux que "votre commentaire a été signalé."
- **Court** - quelques phrases au maximum.
- **Actionnable** - indiquez à l'utilisateur ce qu'il doit changer. "Veuillez modifier votre commentaire pour supprimer l'utilisateur nommé, sinon il sera supprimé."

Vous n'écrivez pas le message vous-même ; c'est l'agent qui le fait, d'après le [prompt initial](#personality-prompt) et les [directives communautaires](#community-guidelines). Votre rôle est d'écrire un prompt qui génère de bons avertissements.

### When to allow it

Pour tout agent de type modération. Le modèle Moderator l'active par défaut.

### Approvals

Moins souvent soumis à une approbation que [Bannir l'utilisateur](#tool-ban-user). Il vaut la peine d'exiger une approbation pendant les premières semaines de vie d'un agent afin de repérer les mauvais avertissements avant qu'ils ne soient envoyés, mais la plupart des opérateurs suppriment cette exigence une fois que l'agent produit des résultats fiables.

### See also

- [Bannir l'utilisateur](#tool-ban-user) - l'étape suivante dans l'escalade.
- [Système de mémoire d'agent](#agent-memory-system) - où sont stockés les enregistrements d'avertissement.

---