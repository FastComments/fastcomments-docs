---
Par défaut, FastComments traduit l'interface du widget (boutons, libellés, messages d'erreur) dans la langue du visiteur, mais les commentaires sont affichés exactement tels qu'ils ont été écrits.

La traduction des commentaires ajoute la traduction des commentaires eux‑mêmes, de sorte qu'une communauté qui écrit en plusieurs langues puisse lire chaque commentaire dans sa propre langue.

Il existe deux modes :

- **Automatique** : les commentaires rédigés dans une langue différente de celle du widget sont affichés traduits. Une ligne « Traduit de l'espagnol » apparaît sous le commentaire avec un lien **Afficher l'original** qui bascule entre la traduction et le texte non traduit.
- **Manuel** : chaque commentaire reçoit un lien **Traduire**. En cliquant dessus, ce commentaire est traduit dans la langue du visiteur, avec le même bascule **Afficher l'original** ensuite. Les commentaires déjà dans la langue du visiteur l'indiquent à la place.

La langue cible est la langue du widget : le paramètre [Locale / Language](#locale) sur la même règle de personnalisation s'il est défini, sinon la langue du navigateur du visiteur. Aucun prérequis n'est demandé au visiteur, pas de compte, d'extension ou de logiciel supplémentaire, et cela fonctionne de la même façon dans le widget intégré et sur les pages de commentaires autonomes.

Cela est activé par <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">règle de personnalisation</a>, de sorte qu'il puisse s'appliquer à l'ensemble de votre site, à un domaine, ou à une seule page. Il est désactivé par défaut.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Liste déroulante Traduction des commentaires sur la page de personnalisation du widget avec les options Désactivé, Automatique et Manuel'; title='Traduction des commentaires' app-screenshot-end]

La traduction est un paramètre côté serveur. Passer `commentTranslation` dans la configuration JavaScript du widget ne l'active pas ; c'est la règle qui le fait.

Chaque commentaire est traduit au maximum une fois par langue cible et le résultat est stocké avec le commentaire, de sorte qu'un commentaire lu par mille hispanophones soit traduit une seule fois. Si l'auteur ou un modérateur modifie le commentaire, la traduction stockée est supprimée et le commentaire est à nouveau traduit lors de la prochaine visualisation. Le texte traduit passe par les mêmes règles de désinfection et de liens que le commentaire original. Les commentaires trop longs, ou dont la langue ne peut être détectée, sont affichés non traduits.

L'utilisation est facturée en jetons LLM au même tarif que les [AI Agents](/guide-ai-agents.html#plans-and-eligibility) et apparaît au même endroit dans vos détails de facturation.
---