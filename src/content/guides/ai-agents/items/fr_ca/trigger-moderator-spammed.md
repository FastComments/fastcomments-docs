Se déclenche lorsqu'un modérateur marque un commentaire comme spam.

### Contexte que reçoit l'agent

- Le commentaire, avec le drapeau post-action `Is Spam`.
- L'**ID utilisateur déclencheur** - le modérateur qui a agi.
- Historique facultatif du fil / de l'utilisateur / contexte de la page tel que configuré.

### Qui déclenche cet événement

Une action d'un modérateur humain. Les marquages de spam initiés par un agent (via [`mark_comment_spam`](#tools-overview)) ne déclenchent **pas** cet événement.

### Utilisations courantes

- **Enregistrement en mémoire** - faire en sorte qu'un agent enregistre une note mémoire à propos de l'utilisateur marqué comme spam (p. ex., "précédemment marqué comme spam pour X par un modérateur") afin que les futurs agents de modération disposent du contexte.
- **Application au niveau de l'utilisateur** - le fait qu'un modérateur marque un commentaire comme spam peut être le signal pour qu'un agent émette également un avertissement ou une courte suspension, soumis à approbation.
- **Miroir vers un système externe** via [Webhooks](#webhooks-overview).

---