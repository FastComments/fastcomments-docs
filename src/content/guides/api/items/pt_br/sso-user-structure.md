FastComments provides an easy to use SSO solution. Atualizar as informações de um usuário com a integração baseada em HMAC é
tão simples quanto fazer o usuário carregar a página com um payload atualizado.

No entanto, pode ser desejável gerenciar um usuário fora desse fluxo, para melhorar a consistência da sua aplicação.

A SSO User API fornece uma forma de CRUD de objetos que chamamos SSOUsers. Esses objetos são diferentes dos Users regulares e
mantidos separados por segurança de tipos.

A estrutura do objeto SSOUser é a seguinte:

[inline-code-attrs-start title = 'Estrutura do SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Permissão de administrador - usuários SSO com esta flag são faturados como Administradores SSO (separados dos usuários SSO regulares)
    isAdminAdmin?: boolean // Permissão de administrador - usuários SSO com esta flag são faturados como Administradores SSO (separados dos usuários SSO regulares)
    isCommentModeratorAdmin?: boolean // Permissão de moderador - usuários SSO com esta flag são faturados como Moderadores SSO (separados dos usuários SSO regulares)
    /** Se null, o Controle de Acesso não será aplicado ao usuário. Se uma lista vazia, este usuário não poderá ver nenhuma página nem @mencionar outros usuários. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Não permita que outros usuários vejam a atividade deste usuário, incluindo comentários, em seu perfil. O padrão é true para fornecer perfis seguros por padrão. **/
    isProfileActivityPrivate?: boolean
    /** Não permita que outros usuários deixem comentários no perfil do usuário, ou vejam comentários de perfil existentes. Padrão false. **/
    isProfileCommentsPrivate?: boolean
    /** Não permita que outros usuários enviem mensagens diretas para este usuário. Padrão false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configuração opcional para badges de usuário. **/
    badgeConfig?: {
        /** Array de IDs de badges para atribuir ao usuário. Limitado a 30 badges. A ordem é respeitada. Estes são badges globais visíveis em todas as páginas. **/
        badgeIds: string[]
        /** Array de IDs de badges com escopo para a página atual (urlId). Esses badges são exibidos apenas na página onde foram atribuídos. **/
        pageBadgeIds?: string[]
        /** Se true, substitui todos os badges exibidos existentes pelos fornecidos. Badges globais e com escopo de página são substituídos independentemente. Se false, adiciona aos badges existentes. **/
        override?: boolean
        /** Se true, atualiza as propriedades de exibição dos badges a partir da configuração do tenant. **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

Usuários SSO são faturados de maneira diferente com base em suas flags de permissão:

- **Regular SSO Users**: Usuários sem permissões de administrador ou moderador são faturados como usuários SSO regulares
- **SSO Admins**: Usuários com `isAccountOwner` ou `isAdminAdmin` flags são faturados separadamente como Administradores SSO (mesma tarifa que administradores regulares do tenant)
- **SSO Moderators**: Usuários com a flag `isCommentModeratorAdmin` são faturados separadamente como Moderadores SSO (mesma tarifa que moderadores regulares)

**Importante**: Para evitar cobrança dupla, o sistema desduplicará automaticamente os usuários SSO em relação aos usuários e moderadores regulares do tenant pelo endereço de email. Se um usuário SSO tiver o mesmo email que um usuário ou moderador regular do tenant, ele não será cobrado duas vezes.

### Access Control

Os usuários podem ser divididos em grupos. É para isso que serve o campo `groupIds`, e é opcional.

### @Mentions

Por padrão `@mentions` usará `username` para buscar outros usuários sso quando o caractere `@` for digitado. Se `displayName` for usado, então resultados correspondentes a
`username` serão ignorados quando houver uma correspondência para `displayName`, e os resultados da busca de `@mention` usarão `displayName`.

### Subscriptions

Com o FastComments, os usuários podem assinar uma página clicando no ícone de sino no widget de comentários e clicando em Inscrever-se.

Com um usuário regular, nós enviamos emails de notificação com base nas configurações de notificação dele.

Com Usuários SSO, nós separamos isso por compatibilidade retroativa. Os usuários só receberão esses emails adicionais de notificação de assinatura
se você definir `optedInSubscriptionNotifications` como `true`.

### Badges

Você pode atribuir badges a usuários SSO usando a propriedade `badgeConfig`. Badges são indicadores visuais que aparecem ao lado do nome de um usuário nos comentários.

- `badgeIds` - Um array de IDs de badge para atribuir ao usuário. Estes são badges globais visíveis em todas as páginas. Devem ser IDs de badge válidos criados na sua conta FastComments. Limitado a 30 badges.
- `pageBadgeIds` - Um array opcional de IDs de badge com escopo para a página atual (`urlId`). Esses badges são exibidos apenas na página onde foram atribuídos. Páginas diferentes podem ter badges com escopo de página diferentes para o mesmo usuário.
- `override` - Se true, todos os badges exibidos existentes serão substituídos pelos fornecidos. Badges globais e com escopo de página são substituídos independentemente — substituir badges globais não afeta badges com escopo de página, e vice-versa. Se false ou omitido, os badges fornecidos serão adicionados aos badges existentes.
- `update` - Se true, as propriedades de exibição dos badges serão atualizadas a partir da configuração do tenant sempre que o usuário fizer login.

---