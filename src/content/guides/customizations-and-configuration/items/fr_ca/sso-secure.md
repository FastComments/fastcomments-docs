[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utilise le chiffrement HMAC-SHA256 comme mécanisme pour implémenter le SSO. D'abord, nous passerons en revue l'architecture globale, fournirons des exemples et des étapes détaillées.

Il existe également de la documentation concernant la migration depuis d'autres fournisseurs ayant des mécanismes SSO similaires, et les différences.

Le flux se présente comme suit :

<div class="screenshot white-bg">
    <div class="title">Flux SSO sécurisé</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagramme SSO sécurisé" />
</div>

Puisque Secure SSO implique du développement full-stack, des exemples de code complets en Java/Spring, NodeJS/Express et PHP vanilla sont actuellement <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">sur GitHub</a>.

Bien que nous utilisions ExpressJS dans l'exemple NodeJS et Spring dans l'exemple Java, aucun framework/bibliothèque n'est requis dans ces environnements d'exécution pour implémenter FastComments SSO — les packages crypto natifs suffisent.

Vous n'avez pas à écrire de nouveaux endpoints d'API avec FastComments SSO. Il suffit de chiffrer les informations de l'utilisateur à l'aide de votre clé secrète et de transmettre le payload au widget de commentaires.

#### Obtenir votre clé secrète d'API

Votre clé secrète d'API peut être récupérée depuis <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">cette page</a>. Vous pouvez aussi accéder à cette page en allant dans Mon compte, en cliquant sur la tuile API/SSO, puis en cliquant sur « Obtenir la clé secrète d'API ».

#### Paramètres du widget de commentaires

La documentation API de haut niveau pour le widget de commentaires se trouve <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ici</a>.

Entrons un peu plus dans le détail de ce que signifient ces paramètres.

Le widget de commentaires prend un objet de configuration - vous le fournissez déjà si vous utilisez FastComments pour transmettre votre id client (appelé tenantId).

Pour activer le SSO, passez un nouvel objet "sso", qui doit contenir les paramètres suivants. Les valeurs doivent être générées côté serveur.

- userDataJSONBase64 : Les données de l'utilisateur au format JSON, puis encodées en Base64.
- verificationHash : Le hachage HMAC-SHA256 créé à partir de UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp : Timestamp epoch, en **millisecondes**. Ne doit pas être dans le futur, ni avoir plus de deux jours dans le passé.
- loginURL : Une URL que le widget de commentaires peut afficher pour connecter l'utilisateur.
- logoutURL : Une URL que le widget de commentaires peut afficher pour déconnecter l'utilisateur.
- loginCallback : Lorsqu'elle est fournie à la place de loginURL, une fonction que le widget de commentaires invoquera lorsqu'on cliquera sur le bouton de connexion.
- logoutCallback : Lorsqu'elle est fournie à la place de logoutURL, une fonction que le widget de commentaires invoquera lorsqu'on cliquera sur le bouton de déconnexion.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### L'objet utilisateur

Le schéma de l'objet User contient les champs suivants :
[inline-code-attrs-start title = 'L’objet utilisateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Requis. 1k caractères maximum. **/
    id: string;
    /** Requis. 1k caractères maximum. Remarque : Doit être unique. **/
    email: string;
    /** Requis. 1k caractères maximum. Remarque : le nom d'utilisateur ne peut pas être un email. N'a pas besoin d'être unique. **/
    username: string;
    /** Optionnel. 3k caractères maximum pour les URL. Par défaut provient de gravatar basé sur email. Prend en charge les images encodées en base64, auquel cas la limite est de 50k caractères. **/ 
    avatar?: string;
    /** Optionnel. Par défaut false. **/
    optedInNotifications?: boolean;
    /** Optionnel. Par défaut false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optionnel. 100 caractères maximum. Ce label sera affiché à côté de leur nom. Par défaut Administrateur/Modérateur lorsque applicable. **/
    displayLabel?: string;
    /** Optionnel. 500 caractères maximum. Sera affiché à la place du nom d'utilisateur. **/
    displayName?: string;
    /** Optionnel. 2k caractères maximum. Le nom de l'utilisateur pointera vers ceci. **/
    websiteUrl?: string;
    /** Optionnel. Jusqu'à 100 groupes par utilisateur. Un id de groupe ne peut pas dépasser 50 caractères. **/
    groupIds?: string[];
    /** Optionnel. Désigne l'utilisateur comme administrateur. **/
    isAdmin?: boolean;
    /** Optionnel. Désigne l'utilisateur comme modérateur. **/
    isModerator?: boolean;
    /** Optionnel, par défaut true. Réglez sur false pour activer l'onglet « activity » dans le profil de l'utilisateur. **/
    isProfileActivityPrivate?: boolean;
    /** Optionnel, par défaut false. Réglez sur true pour désactiver les commentaires de profil. **/
    isProfileCommentsPrivate?: boolean;
    /** Optionnel, par défaut false. Réglez sur true pour désactiver l'envoi de messages directs à cet utilisateur. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Modérateurs et administrateurs

Pour les admins et les modérateurs, passez les flags respectifs `isAdmin` ou `isModerator` dans l'objet `SSOUser`.

#### Notifications

Pour activer ou désactiver les notifications, définissez la valeur de `optedInNotifications` sur `true` ou `false` respectivement. La première fois que l'utilisateur charge la page avec cette valeur dans le payload SSO, ses paramètres de notification seront mis à jour.

De plus, si vous souhaitez que les utilisateurs reçoivent des courriels de notification pour l'activité sur les pages auxquelles ils se sont abonnés (plutôt que seulement des notifications dans l'application), définissez `optedInSubscriptionNotifications` sur `true`.

#### Utilisateurs VIP et étiquettes spéciales

Vous pouvez afficher une étiquette spéciale à côté du nom de l'utilisateur en utilisant le champ optionnel "displayLabel".

#### Utilisateurs non authentifiés

Pour représenter un utilisateur non authentifié, il suffit de ne pas remplir userDataJSONBase64, verificationHash ou timestamp. Fournissez un loginURL.

Ces utilisateurs ne pourront pas commenter, et se verront présenter un message de connexion (message, lien ou bouton, selon la configuration).

#### Exemples directs pour la sérialisation et le hachage des données utilisateur

Plus de détails et des exemples sont disponibles <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ici</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ici</a> (java) et <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ici</a> (php).

Nous comprenons que toute intégration peut être un processus compliqué et pénible. N'hésitez pas à contacter votre représentant ou à utiliser la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">page d'assistance</a>.