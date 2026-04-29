---
Se déclenche lorsqu'un commentaire est automatiquement signalé comme spam par le moteur anti-spam intégré de FastComments - **pas** par un modérateur et pas par un autre agent.

### Contexte que reçoit l'agent

- Le commentaire automatiquement signalé comme spam.
- Historique facultatif du fil / de l'utilisateur / contexte de la page tel que configuré.

### Qui déclenche ceci

Le pipeline anti-spam de la plateforme. Voir [Détection du spam](/guide-moderation.html#spam-detection) dans le guide de modération pour plus de détails.

### Utilisations courantes

- **Modération en seconde lecture** - le moteur anti-spam a un rappel élevé mais une précision imparfaite ; un agent entraîné au style spécifique de votre communauté peut détecter les faux positifs. L'agent peut demander à retirer le marquage d'un commentaire mal classé.
- **Débannissement automatisé** - si votre locataire bannit de manière agressive les nouveaux comptes pour spam, un agent lié à ce déclencheur peut réviser et lever les faux positifs évidents avant qu'un humain ne les voie.

### Remarques

- Le déclencheur ne se déclenche **pas** pour le spam marqué par un modérateur (utilisez [Déclencheur : Spam marqué par un modérateur](#trigger-moderator-spammed)) ni pour le spam marqué par un autre agent.
- Un commentaire automatiquement signalé comme spam puis marqué comme non-spam par un modérateur ne déclenchera pas de nouveau le déclencheur.
- S'abonner à ce déclencheur est le plus utile dans les locataires où le mode auto-spam du moteur est activé dans les Paramètres de modération. Sinon, le déclencheur ne se déclenchera pas.

---