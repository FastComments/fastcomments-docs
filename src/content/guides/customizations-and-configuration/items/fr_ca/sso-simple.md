[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Avec Simple SSO, nous pouvons fournir au widget de commentaires des informations sur l'utilisateur afin qu'il n'ait pas à saisir son nom d'utilisateur ou son courriel pour commenter.

Nous pouvons configurer Simple SSO comme suit :

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

L'utilisateur sera connecté, et un utilisateur SSO sera créé en coulisses. La propriété `createdFromSimpleSSO` de l'utilisateur sera définie à `true` s'il est récupéré via l'API.

Notes: 

- Le courriel est l'identifiant unique pour Simple SSO.
- Fournir un courriel avec Simple SSO n'est pas requis, toutefois, par défaut leurs commentaires apparaîtront comme "Non vérifié". <b>Si aucun courriel n'est fourni, l'utilisateur ne peut pas être entièrement authentifié.</b>
- **NOUVEAU** Depuis janvier 2022 : les noms d'utilisateur n'ont pas besoin d'être uniques sur l'ensemble de fastcomments.com
- Simple SSO peut créer et mettre à jour automatiquement des utilisateurs SSO si un courriel est fourni et que l'utilisateur n'a pas été créé initialement par Secure SSO.
- Vous pouvez spécifier des badges pour l'utilisateur avec la propriété `badgeConfig`. Le tableau `badgeIds` contient les IDs des badges globaux à associer à l'utilisateur. Le tableau `pageBadgeIds` contient des IDs de badges limités à la page actuelle (`urlId`) — ces badges sont affichés uniquement sur la page où ils ont été attribués. Si `override` est défini à `true`, il remplacera les badges affichés existants (les badges globaux et ceux limités à la page sont remplacés indépendamment) ; si `false`, il ajoutera aux badges existants.

---