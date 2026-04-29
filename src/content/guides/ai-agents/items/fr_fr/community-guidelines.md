Le champ **Community guidelines** du formulaire d'édition est un bloc de texte de politique optionnel inclus dans le message de contexte du rôle utilisateur à chaque exécution pour cet agent. Il est encadré comme texte non fiable (le même encadrement que la plateforme applique aux corps de commentaire et aux autres contenus fournis par les utilisateurs), donc le modèle le considère comme une référence de politique, et non comme des instructions système. C'est l'endroit canonique pour écrire « quel comportement est autorisé ou non sur ce site » afin que l'agent l'applique de manière cohérente.

### En quoi cela diffère de l'invite initiale

- **Invite initiale** - le rôle de l'agent et son style de prise de décision. "Vous êtes un modérateur. Préférez l'avertissement à l'interdiction."
- **Community guidelines** - les règles de votre communauté, rédigées en langage de politique. "Pas d'attaques personnelles. Pas de liens promotionnels provenant de comptes de moins de 24 heures. Les commentaires hors sujet peuvent être supprimés si un fil est enflammé."

Les deux font partie de la même fenêtre de contexte, mais elles entrent à différents niveaux - l'invite initiale fait partie du rôle système, le document de directives est du texte encadré à l'intérieur du message de contexte du rôle utilisateur. Cette séparation facilite la modification quand vous voulez mettre à jour l'une sans relire l'autre.

### Qu'est-ce qu'un bon document de directives

Un document court, précis, rédigé par un humain. Les listes fonctionnent mieux que la prose :

[inline-code-attrs-start title = 'Exemple de directives communautaires'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

L'agent applique ceci à chaque exécution. Si vous modifiez les directives, le changement prend effet au déclenchement suivant - les exécutions passées ne sont pas réévaluées rétroactivement.

### Ce qu'il ne faut pas mettre ici

- **Instructions de formatage de sortie** ("répondez en HTML", "utilisez des emoji"). Cela appartient à l'[invite initiale](#personality-prompt).
- **Texte localisé.** Le document de directives, comme l'invite, est **exclusivement en anglais** pour la même raison - la traduction automatique peut changer le comportement de l'agent sans le signaler. Si vous avez des politiques qui varient selon la locale, rédigez-les toutes en anglais dans ce même document et structurez-le comme « pour les pages en allemand : ... »
- **Longues citations de politiques externes.** Paraphrasez. Un long contexte coûte des tokens à chaque exécution.
- **Informations personnelles identifiables ou secrets.** Ce texte est envoyé au fournisseur LLM à chaque exécution.

### Longueur

Le champ est limité à **4000 caractères** (appliqué à la fois par le formulaire et la route d'enregistrement). Le coût en tokens à chaque exécution est proportionnel à la longueur, donc même dans la limite quelques centaines de mots suffisent généralement. Si vos politiques communautaires occupent plusieurs pages, résumez les parties nécessaires à l'agent dans un extrait spécifiquement prévu pour ce champ.

### Versioning

Il n'y a pas d'historique de version intégré pour le document de directives - la dernière valeur enregistrée est celle que l'agent utilise. Si vous voulez un historique, copiez le document dans votre propre système de suivi avant chaque modification majeure. Le flux [Refine Prompts](#refining-prompts) peut enregistrer les changements de l'*invite initiale* mais ne versionne pas le document de directives.