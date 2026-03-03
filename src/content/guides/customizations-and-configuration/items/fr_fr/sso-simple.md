[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Avec Simple SSO, nous pouvons fournir au widget de commentaires des informations sur l'utilisateur afin qu'il n'ait pas à saisir son nom d'utilisateur ou son e-mail pour commenter.

Nous pouvons configurer Simple SSO comme suit :

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

L'utilisateur sera connecté, et un utilisateur SSO sera créé en arrière-plan. L'utilisateur aura `createdFromSimpleSSO` défini sur `true` s'il est récupéré depuis l'API.

Notes: 

- L'adresse e-mail est l'identifiant unique pour Simple SSO.
- Fournir une adresse e-mail avec Simple SSO n'est pas obligatoire ; toutefois, par défaut, leurs commentaires s'afficheront comme "Non vérifié". <b>Si aucune adresse e-mail n'est fournie, l'utilisateur ne peut pas être entièrement authentifié.</b>
- **NOUVEAU** Depuis janvier 2022 : les noms d'utilisateur n'ont pas à être uniques sur l'ensemble de fastcomments.com
- Simple SSO peut créer et mettre à jour automatiquement des utilisateurs SSO, si une adresse e-mail est fournie, et si l'utilisateur n'a pas été initialement créé via Secure SSO.
- Vous pouvez spécifier des badges pour l'utilisateur avec la propriété `badgeConfig`. Le tableau `badgeIds` contient les identifiants des badges globaux à associer à l'utilisateur. Le tableau `pageBadgeIds` contient les identifiants de badges limités à la page courante (`urlId`) — ces badges ne sont affichés que sur la page où ils ont été attribués. Si `override` est défini sur `true`, cela remplacera les badges affichés existants (les badges globaux et ceux limités à la page sont remplacés indépendamment) ; si `false`, ils seront ajoutés aux badges existants.

---