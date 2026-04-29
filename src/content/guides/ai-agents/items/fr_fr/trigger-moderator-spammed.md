Se déclenche lorsqu'un modérateur marque un commentaire comme spam.

### Contexte que reçoit l'agent

- Le commentaire, avec le drapeau post-action `Is Spam`.
- L'**ID de l'utilisateur déclencheur** - le modérateur qui a agi.
- Historique facultatif du fil / de l'utilisateur / contexte de la page selon la configuration.

### Qui déclenche ceci

Action effectuée par un modérateur humain. Les marquages de spam initiés par un agent (via [`mark_comment_spam`](#tools-overview)) ne déclenchent **pas** ce déclencheur.

### Utilisations courantes

- **Enregistrement de mémoire** - demander à un agent d'enregistrer une note mémoire au sujet de l'utilisateur signalé comme spam (par exemple, "précédemment signalé comme spam pour X par modérateur") afin que les agents de modération futurs aient le contexte.
- **Application au niveau utilisateur** - le fait qu'un modérateur marque un commentaire comme spam peut être le signal pour qu'un agent émette aussi un avertissement ou une suspension temporaire, soumis à approbation.
- **Mise en miroir vers un système externe** via [Webhooks](#webhooks-overview).

---