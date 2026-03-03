---
[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Com o Simple SSO, podemos fornecer ao widget de comentários informações sobre o usuário para que ele não precise inserir seu nome de usuário ou e-mail para comentar.

Podemos configurar o Simple SSO da seguinte forma:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

O usuário será logado, e um Usuário SSO será criado em segundo plano. O usuário terá `createdFromSimpleSSO` definido como `true` se obtido pela API.

Notas: 

- E-mail é o identificador único para o Simple SSO.
- Fornecer um e-mail com o Simple SSO não é obrigatório, no entanto por padrão os comentários aparecerão como "Não verificado". <b>Se nenhum e-mail for fornecido, o usuário não poderá ser totalmente autenticado.</b>
- **NOVO** Since Jan 2022: Nomes de usuário não precisam ser únicos em todo o fastcomments.com
- O Simple SSO pode criar e atualizar automaticamente usuários SSO, se um e-mail for fornecido, e o usuário não tiver sido originalmente criado pelo Secure SSO.
- Você pode especificar badges para o usuário com a propriedade `badgeConfig`. O array `badgeIds` contém os IDs de badges globais a associar ao usuário. O array `pageBadgeIds` contém IDs de badges com escopo para a página atual (`urlId`) — esses badges são exibidos apenas na página em que foram atribuídos. Se `override` estiver definido como `true`, ele substituirá os badges exibidos existentes (globais e com escopo de página são sobrescritos independentemente); se `false`, ele será adicionado aos badges existentes.

---