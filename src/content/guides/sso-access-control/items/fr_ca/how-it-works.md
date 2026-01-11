---
Le contrôle d'accès de FastComments fonctionne en assignant `Pages` et `Users` aux groupes souhaités.

Un groupe est simplement un identifiant de chaîne, comme `GREEN` ou `abc-123`.

`Users` et `Pages` ne sont pas limités à un seul groupe. Ils sont limités à `100` et `1000` groupes, respectivement. 

#### Accès aux pages non autorisées

Si un utilisateur tente d'accéder à une page à laquelle il n'a pas accès, il verra un message d'erreur, comme ci-dessous :

<div class="screenshot white-bg">
    <div class="title">Exemple d'échec d'autorisation</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Authorization Failure Example" />
</div>

Le texte du message peut être personnalisé.

---