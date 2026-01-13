[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Com o Simple SSO, podemos fornecer ao widget de comentários informações sobre o usuário para que ele não precise inserir seu nome de usuário ou e-mail para comentar.

Podemos configurar o Simple SSO da seguinte forma:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

O usuário será autenticado, e um Usuário SSO será criado nos bastidores. O usuário terá `createdFromSimpleSSO` definido como `true` se buscado pela API.

Notes: 

- O e-mail é o identificador único para o Simple SSO.
- Fornecer um e-mail com o Simple SSO não é obrigatório, entretanto por padrão os comentários deles aparecerão como "Não verificado". <b>Se nenhum e-mail for fornecido, o usuário não poderá ser totalmente autenticado.</b>
- **NEW** Desde Jan 2022: Nomes de usuário não precisam ser exclusivos em todo o fastcomments.com
- O Simple SSO pode criar e atualizar usuários SSO automaticamente, se um e-mail for fornecido, e o usuário não tiver sido originalmente criado a partir do Secure SSO.
- Você pode especificar badges para o usuário com a propriedade `badgeConfig`. O array `badgeIds` contém os IDs dos badges a serem associados ao usuário. Se `override` estiver definido como `true`, ele substituirá todos os badges existentes exibidos nos comentários; se `false`, ele adicionará aos badges existentes.