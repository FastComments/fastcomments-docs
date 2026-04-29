Le champ **Lignes directrices communautaires** du formulaire de modification est un bloc de texte de politique optionnel inclus dans le message de contexte lié au rôle d'utilisateur à chaque exécution pour cet agent. Il est encadré comme texte non fiable (le même encadrement que la plateforme applique aux corps de commentaires et aux autres contenus fournis par les utilisateurs), donc le modèle le traite comme référence de politique, pas comme instruction système. C'est l'endroit canonique pour consigner « quel comportement est et n'est pas permis sur ce site » afin que l'agent l'applique de façon cohérente.

### Comment cela diffère de l'invite initiale

- **Invite initiale** - le rôle de l'agent et le style de prise de décision. « Vous êtes modérateur. Préférez l'avertissement au bannissement. »
- **Lignes directrices communautaires** - les règles de votre communauté, formulées en langage de politique. « Pas d'attaques personnelles. Pas de liens promotionnels provenant de comptes de moins de 24 heures. Les commentaires hors sujet peuvent être supprimés si un fil est enflammé. »

Les deux alimentent la même fenêtre de contexte, mais elles entrent à des niveaux différents - l'invite initiale fait partie du rôle système, le document de lignes directrices est un texte encadré à l'intérieur du message de contexte lié au rôle d'utilisateur. Cette séparation facilite la modification lorsque vous voulez mettre à jour l'une sans relire l'autre.

### À quoi ressemble un bon document de lignes directrices

Un document court, précis, rédigé par un humain. Les listes fonctionnent mieux que la prose :

[inline-code-attrs-start title = 'Exemple de lignes directrices communautaires'; type='text' inline-code-attrs-end]
[inline-code-start]
Autorisé :
- Désaccord substantiel, même formulé de manière forte.
- Liens vers des sources originales, même depuis des comptes récents.
- Apartés hors sujet si le fil parent le permet.

Interdit :
- Attaques personnelles contre des utilisateurs nommément identifiés.
- Doxxing ou partage d'informations privées.
- Activité promotionnelle coordonnée (plusieurs commentaires faisant la promotion du même lien externe).
- Commentaires visant uniquement à faire dérailler la discussion.

Limite :
- Langage fort sans cible. Autorisé s'il n'est pas dirigé contre une personne.
- Sujets politiques hors du sujet de la page. Hors sujet ; avertir d'abord, ne pas supprimer sauf en cas de persistance.
[inline-code-end]

L'agent applique cela à chaque exécution. Si vous modifiez les lignes directrices, la modification prend effet au déclenchement suivant - les exécutions passées ne sont pas réévaluées rétroactivement.

### Ce qu'il ne faut pas mettre ici

- **Instructions de formatage de sortie** ("répondre en HTML", "utiliser des émojis"). Celles-ci appartiennent à l'[invite initiale](#personality-prompt).
- **Texte localisé.** Le document de lignes directrices, comme l'invite, est **réservé à l'anglais** pour la même raison — la traduction automatique peut modifier silencieusement le comportement de l'agent. Si vous avez des politiques qui varient selon la locale, rédigez-les toutes en anglais dans ce même document et structurez le document comme "for German-language pages: ..."
- **Longues citations de politiques externes.** Paraphrasez. Un contexte long coûte des tokens à chaque exécution.
- **PII ou secrets.** Ce texte est envoyé au fournisseur de LLM à chaque exécution.

### Longueur

Le champ est plafonné à **4000 caractères** (appliqué à la fois par le formulaire et par la route d'enregistrement). Le coût en tokens à chaque exécution est proportionnel à la longueur, donc même dans la limite, quelques centaines de mots suffisent généralement. Si vos politiques communautaires s'étendent sur plusieurs pages, résumez les parties dont l'agent a besoin dans une spécification dédiée à ce champ.

### Gestion des versions

Il n'y a pas d'historique de versions intégré pour le document de lignes directrices — la dernière valeur enregistrée est celle utilisée par l'agent. Si vous voulez un historique, copiez le document dans votre propre système de suivi avant chaque modification majeure. Le flux [Refine Prompts](#refining-prompts) peut enregistrer les changements de l'*invite initiale* mais n'assure pas la gestion de versions pour le document de lignes directrices.

---