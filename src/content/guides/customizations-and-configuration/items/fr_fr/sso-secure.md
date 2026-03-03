[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utilise le chiffrement HMAC-SHA256 comme mécanisme pour implémenter le SSO. Nous allons d'abord passer en revue l'architecture globale, fournir des exemples et des étapes détaillées.

Il existe également de la documentation concernant la migration depuis d'autres fournisseurs ayant des mécanismes SSO similaires, ainsi que les différences.

Le flux ressemble à ceci :

<div class="screenshot white-bg">
    <div class="title">Flux SSO sécurisé</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Schéma SSO sécurisé" />
</div>

Étant donné que Secure SSO implique du développement full-stack, des exemples de code complets en Java/Spring, NodeJS/Express et PHP vanille sont actuellement <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">sur GitHub</a>.

Bien que nous utilisions ExpressJS dans l'exemple NodeJS et Spring dans l'exemple Java, aucun framework/bibliothèque n'est requis dans ces environnements d'exécution pour implémenter FastComments SSO - les packages crypto natifs suffisent.

Vous n'avez pas à créer de nouveaux endpoints API avec FastComments SSO. Il suffit de chiffrer les informations de l'utilisateur en utilisant votre clé secrète et de transmettre le payload au widget de commentaires.

#### Récupérer votre clé secrète API

Votre clé secrète API peut être récupérée depuis <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">cette page</a>. Vous pouvez aussi trouver cette page en allant dans Mon compte, en cliquant sur la tuile API/SSO, puis en cliquant sur « Obtenir la clé secrète de l'API ».

#### Paramètres du widget de commentaires

La documentation API de haut niveau pour le widget de commentaires se trouve <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ici</a>.

Entrons dans plus de détails sur la signification de ces paramètres.

Le widget de commentaires prend un objet de configuration - vous lui passez déjà ceci si vous utilisez FastComments pour fournir votre id client (appelé tenantId).

Pour activer le SSO, fournissez un nouvel objet "sso", qui doit contenir les paramètres suivants. Les valeurs doivent être générées côté serveur.

- userDataJSONBase64: Les données de l'utilisateur au format JSON, puis encodées en Base64.
- verificationHash: Le hachage HMAC-SHA256 créé à partir de UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Timestamp epoch, en **millisecondes**. Ne doit pas être dans le futur, ni de plus de deux jours dans le passé.
- loginURL: Une URL que le widget de commentaires peut afficher pour connecter l'utilisateur.
- logoutURL: Une URL que le widget de commentaires peut afficher pour déconnecter l'utilisateur.
- loginCallback: Lorsqu'elle est fournie au lieu de loginURL, une fonction que le widget de commentaires invoquera lors du clic sur le bouton de connexion.
- logoutCallback: Lorsqu'elle est fournie au lieu de logoutURL, une fonction que le widget de commentaires invoquera lors du clic sur le bouton de déconnexion.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### L'objet utilisateur

Le schéma de l'objet User contient les éléments suivants :
[inline-code-attrs-start title = 'L'objet utilisateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obligatoire. 1k caractères max. **/
    id: string;
    /** Obligatoire. 1k caractères max. Remarque : doit être unique. **/
    email: string;
    /** Obligatoire. 1k caractères max. Remarque : Le nom d'utilisateur ne peut pas être un e-mail. N'a pas besoin d'être unique. **/
    username: string;
    /** Optionnel. 3k caractères max pour les URL. Par défaut, provient de gravatar basé sur l'e-mail. Prend en charge les images encodées en base64, auquel cas la limite est de 50k caractères. **/ 
    avatar?: string;
    /** Optionnel. Par défaut false. **/
    optedInNotifications?: boolean;
    /** Optionnel. Par défaut false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optionnel. 100 caractères max. Ce label sera affiché à côté de leur nom. Par défaut Administrateur/Modérateur lorsque applicable. **/
    displayLabel?: string;
    /** Optionnel. 500 caractères max. Ceci sera affiché à la place du nom d'utilisateur. **/
    displayName?: string;
    /** Optionnel. 2k caractères max. Le nom de l'utilisateur sera lié à ceci. **/
    websiteUrl?: string;
    /** Optionnel. Jusqu'à 100 groupes par utilisateur. Un id de groupe ne peut pas dépasser 50 caractères. **/
    groupIds?: string[];
    /** Optionnel. Désigne l'utilisateur comme administrateur. **/
    isAdmin?: boolean;
    /** Optionnel. Désigne l'utilisateur comme modérateur. **/
    isModerator?: boolean;
    /** Optionnel, par défaut true. Mettre à false pour activer l'onglet « activité » dans le profil de l'utilisateur. **/
    isProfileActivityPrivate?: boolean;
    /** Optionnel, par défaut false. Mettre à true pour désactiver les commentaires sur le profil. **/
    isProfileCommentsPrivate?: boolean;
    /** Optionnel, par défaut false. Mettre à true pour désactiver la messagerie directe pour cet utilisateur. **/
    isProfileDMDisabled?: boolean;
    /** Configuration optionnelle pour les badges utilisateur. **/
    badgeConfig?: {
        /** Tableau d'ID de badges globaux à attribuer. Limité à 30 badges. L'ordre est respecté. **/
        badgeIds: string[];
        /** Tableau d'ID de badges limité à la page courante (urlId). Affiché uniquement sur la page assignée. **/
        pageBadgeIds?: string[];
        /** Si true, remplace les badges affichés existants. Les badges globaux et ceux spécifiques à une page sont remplacés indépendamment. **/
        override?: boolean;
        /** Si true, met à jour les propriétés d'affichage des badges à partir de la configuration du tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Modérateurs et administrateurs

Pour les administrateurs et les modérateurs, passez les indicateurs `isAdmin` ou `isModerator` respectifs dans l'objet `SSOUser`.

#### Notifications

Pour activer ou désactiver les notifications, définissez la valeur de `optedInNotifications` sur `true` ou `false` respectivement. La première fois que l'utilisateur charge la page avec cette valeur dans le payload SSO, ses paramètres de notification seront mis à jour.

De plus, si vous souhaitez que les utilisateurs reçoivent des e-mails de notification pour l'activité sur les pages auxquelles ils se sont abonnés (par opposition aux notifications uniquement dans l'application), définissez `optedInSubscriptionNotifications` sur `true`.

#### Utilisateurs VIP et labels spéciaux

Vous pouvez afficher un label spécial à côté du nom de l'utilisateur en utilisant le champ optionnel "displayLabel".

#### Utilisateurs non authentifiés

Pour représenter un utilisateur non authentifié, il suffit de ne pas remplir userDataJSONBase64, verificationHash ou timestamp. Fournissez un loginURL.

Ces utilisateurs ne pourront pas commenter, et verront plutôt un message de connexion (message, lien ou bouton, selon la configuration).

#### Exemples directs pour la sérialisation et le hachage des données utilisateur

Plus de détails et des exemples <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ici</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ici</a> (java) et <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ici</a> (php).

Nous comprenons que toute intégration peut être un processus compliqué et pénible. N'hésitez pas à contacter votre représentant ou à utiliser la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">page d'assistance</a>.