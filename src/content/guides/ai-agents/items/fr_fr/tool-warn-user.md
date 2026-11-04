L'outil Warn envoie un avertissement privé en DM à un utilisateur au sujet d'un commentaire spécifique, et enregistre en même temps l'avertissement dans la [mémoire d'agent](#agent-memory-system) partagée. Les deux écritures sont atomiques - l'utilisateur ne voit jamais un avertissement qui n'est pas également consigné.

### Pourquoi il existe

La politique d'escalade de la plateforme est **avertir d'abord, bannir uniquement si l'utilisateur récidive**. L'outil Warn est ce qui rend cette politique actionnable : il donne à l'utilisateur une chance de se corriger, et c'est l'enregistrement de l'avertissement qu'un futur agent trouvera lorsqu'il cherchera dans la mémoire avant d'envisager un bannissement.

L'outil déduplique également : si l'agent a déjà émis un avertissement au même utilisateur au sujet du même commentaire, un second avertissement est sans effet. Ainsi, un LLM qui boucle ou se déclenche à nouveau sur le même commentaire ne peut pas spammer l'utilisateur avec plusieurs avertissements.

### Ce qui doit figurer dans l'avertissement

Un message court (limité à 1000 caractères) montré à l'utilisateur sous forme de DM. Les bons avertissements sont :

- **Spécifiques** - « Les attaques personnelles contre des utilisateurs nommés ne sont pas autorisées dans cette communauté » est plus efficace que « votre commentaire a été signalé ».
- **Courts** - quelques phrases au maximum.
- **Actionnables** - indiquez à l'utilisateur ce qu'il doit changer. « Veuillez modifier votre commentaire pour retirer l'utilisateur nommé, sinon il sera supprimé. »

Vous n'écrivez pas le message vous-même ; c'est l'agent qui le fait, en se basant sur le [prompt initial](#personality-prompt) et les [règles de la communauté](#community-guidelines). Votre rôle est d'écrire un prompt qui produit de bons avertissements.

### Quand l'autoriser

Pour tout agent de type modération. Le template Moderator l'active par défaut.

### Approbations

Moins souvent soumis à approbation que [Ban user](#tool-ban-user). Il est utile de l'activer pendant les premières semaines de vie d'un agent afin de repérer les mauvais avertissements avant leur envoi, mais la plupart des opérateurs suppriment cette approbation une fois que l'agent produit des résultats fiables.

### Voir aussi

- [Ban user](#tool-ban-user) - l'étape suivante dans l'escalade.
- [Agent Memory System](#agent-memory-system) - là où sont conservés les enregistrements d'avertissements.