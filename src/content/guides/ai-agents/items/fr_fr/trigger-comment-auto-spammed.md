Se déclenche lorsqu'un commentaire est automatiquement signalé comme spam par le moteur anti-spam intégré de FastComments - **pas** par un modérateur et pas par un autre agent.

### Contexte reçu par l'agent

- Le commentaire signalé automatiquement comme spam.
- Fil / historique de l'utilisateur / contexte de la page optionnels, selon la configuration.

### Qui le déclenche

Le pipeline anti-spam de la plateforme. Voir [Détection du spam](/guide-moderation.html#spam-detection) dans le guide de modération pour plus de détails.

### Utilisations courantes

- **Modération en seconde lecture** - le moteur anti-spam a un fort rappel mais une précision imparfaite ; un agent entraîné sur le style spécifique de votre communauté peut détecter les faux positifs. L'agent peut appeler une action pour lever le signalement d'un commentaire mal classé.
- **Désbannissement automatisé** - si votre tenant bannit agressivement les nouveaux comptes pour spam, un agent déclenché peut examiner et lever les faux positifs évidents avant qu'un humain ne les voie.

### Remarques

- Le déclencheur **ne** se déclenche **pas** pour le spam marqué par un modérateur (utilisez [Déclencheur : Spam marqué par un modérateur](#trigger-moderator-spammed)) ni pour le spam marqué par un autre agent.
- Un commentaire automatiquement signalé comme spam puis ultérieurement marqué Not Spam par un modérateur ne réactive pas le déclencheur.
- S'abonner à ce déclencheur est le plus utile dans les tenants où le mode auto-spam du moteur est activé dans les paramètres de modération. Sinon, le déclencheur ne se déclenchera pas.