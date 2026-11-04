[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Com o Simple SSO, podemos fornecer ao widget de comentários informações sobre o usuário para que ele não precise inserir seu nome de usuário ou e‑mail para comentar.

Podemos configurar o Simple SSO da seguinte forma:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

O usuário será autenticado e criará um Usuário SSO nos bastidores. O usuário terá `createdFromSimpleSSO` definido como `true` se for obtido da API.

Observações:

- E‑mail é o identificador único para o Simple SSO.
- Fornecer um e‑mail com Simple SSO não é obrigatório, porém, por padrão, os comentários deles aparecerão como "Unverified". <b>Se nenhum e‑mail for fornecido, o usuário não pode ser totalmente autenticado.</b>
- **NOVO** Desde Jan 2022: Nomes de usuário não precisam ser únicos em todo o fastcomments.com
- O Simple SSO pode criar e atualizar usuários SSO automaticamente, se um e‑mail for fornecido, e o usuário não foi originalmente criado a partir do Secure SSO. Se um payload Simple SSO nomear um usuário criado pelo Secure SSO ou pela API, esse usuário permanecerá inalterado e o visitante receberá um usuário separado.
- Você pode especificar emblemas para o usuário com a propriedade `badgeConfig`. O array `badgeIds` contém os IDs dos emblemas globais a serem associados ao usuário. O array `pageBadgeIds` contém IDs de emblemas limitados à página atual (`urlId`) — esses emblemas são exibidos apenas na página onde foram atribuídos. Se `override` for definido como `true`, ele substituirá os emblemas exibidos existentes (globais e de página são sobrescritos independentemente); se for `false`, ele será adicionado aos emblemas existentes.

#### Configurações do Simple SSO

O Simple SSO está sempre disponível. Dois ajustes controlam o quanto o FastComments confia nele. Você pode encontrá-los pesquisando por SSO em uma <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regra de personalização</a>. Ambos estão desativados por padrão, e se aplicam independentemente de "Require SSO To View Comments" estar ativado.

Contas que já utilizavam Simple SSO quando essas configurações foram adicionadas têm ambas ativadas, portanto nada mudou para elas.

Somente a regra de personalização que melhor corresponde a uma página é aplicada, então, se você tiver mais de uma regra, defina-as em cada uma delas. Uma nova regra começa com elas ativadas se alguma das suas outras regras as tiver ativado.

##### Verificar Comentários Automaticamente via Simple SSO

Quando isso está ativado, os comentários de usuários Simple SSO são verificados imediatamente.

Quando isso está desativado, os comentários de usuários Simple SSO aparecem como "Unverified" até que o usuário clique em um link que enviamos por e‑mail para o endereço no payload Simple SSO. Se você aprova automaticamente apenas comentários verificados, eles também aguardam esse link. Se não houver e‑mail no payload, o comentário permanece não verificado.

Esta configuração afeta apenas comentários. Votos de usuários Simple SSO são sempre contabilizados imediatamente.

##### Permitir Administradores Simple SSO

Quando isso está ativado, um usuário Simple SSO pode ser administrador ou moderador da sua conta:

- Um usuário Simple SSO que tem permissões de administrador ou moderador as mantém.
- Um usuário Simple SSO com o mesmo e‑mail de um dos seus administradores ou moderadores assume suas permissões.

Quando isso está desativado, o Simple SSO não concede essas permissões. Um payload Simple SSO que nomeia um usuário que as possui gera um usuário separado sem permissões, e o usuário original permanece inalterado.

Os payloads Simple SSO não são assinados, portanto, quando isso está ativado, qualquer pessoa que conheça o e‑mail ou ID de usuário de um administrador pode agir como ele. Se isso for importante para o seu site, deixe desativado e use Secure SSO para seus administradores.

Usuários criados pelo Secure SSO ou pela API não podem ser alcançados pelo Simple SSO, independentemente de isso estar ativado ou não.

---