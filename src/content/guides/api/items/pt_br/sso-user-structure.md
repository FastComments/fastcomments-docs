O FastComments fornece uma solução de SSO fácil de usar. Atualizar as informações de um usuário com a integração baseada em HMAC é
tão simples quanto fazer o usuário carregar a página com um payload atualizado.

No entanto, pode ser desejável gerenciar um usuário fora desse fluxo, para melhorar a consistência da sua aplicação.

A API SSO User fornece uma forma de realizar CRUD em objetos que chamamos de SSOUsers. Esses objetos são diferentes dos Users regulares e
são mantidos separados por segurança de tipos.

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
    isAccountOwner?: boolean // Permissão de Admin - usuários SSO com esta flag são cobrados como SSO Admins (separados dos usuários SSO regulares)
    isAdminAdmin?: boolean // Permissão de Admin - usuários SSO com esta flag são cobrados como SSO Admins (separados dos usuários SSO regulares)
    isCommentModeratorAdmin?: boolean // Permissão de Moderador - usuários SSO com esta flag são cobrados como SSO Moderadores (separados dos usuários SSO regulares)
    /** Se null, o Controle de Acesso não será aplicado ao usuário. Se for uma lista vazia, este usuário não poderá ver nenhuma página nem @mencionar outros usuários. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Não permite que outros usuários vejam a atividade deste usuário, incluindo comentários, em seu perfil. O padrão é true para fornecer perfis seguros por padrão. **/
    isProfileActivityPrivate?: boolean
    /** Não permite que outros usuários deixem comentários no perfil do usuário, ou vejam os comentários existentes do perfil. Padrão é false. **/
    isProfileCommentsPrivate?: boolean
    /** Não permite que outros usuários enviem mensagens diretas para este usuário. Padrão é false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configuração opcional para emblemas do usuário. **/
    badgeConfig?: {
        /** Array de IDs de emblemas a serem atribuídos ao usuário. Limitado a 30 emblemas. A ordem é respeitada. Estes são emblemas globais visíveis em todas as páginas. **/
        badgeIds: string[]
        /** Array de IDs de emblemas com escopo na página atual (urlId). Esses emblemas são exibidos apenas na página onde foram atribuídos. **/
        pageBadgeIds?: string[]
        /** Se true, substitui todos os emblemas exibidos existentes pelos fornecidos. Emblemas globais e com escopo de página são substituídos de forma independente. Se false, adiciona aos emblemas existentes. **/
        override?: boolean
        /** Se true, atualiza as propriedades de exibição dos emblemas a partir da configuração do tenant. **/
        update?: boolean
    }
}
[inline-code-end]

### Cobrança de Usuários SSO

Os usuários SSO são cobrados de forma diferente com base em suas flags de permissão:

- **Usuários SSO Regulares**: Usuários sem permissões de admin ou moderador são cobrados como usuários SSO regulares
- **SSO Admins**: Usuários com as flags `isAccountOwner` ou `isAdminAdmin` são cobrados separadamente como SSO Admins (mesma tarifa que admins de tenant regulares)
- **SSO Moderadores**: Usuários com a flag `isCommentModeratorAdmin` são cobrados separadamente como SSO Moderadores (mesma tarifa que moderadores regulares)

**Importante**: Para evitar cobrança duplicada, o sistema deduplica automaticamente usuários SSO em relação a usuários e moderadores de tenant regulares por endereço de e-mail. Se um usuário SSO tiver o mesmo e-mail que um usuário ou moderador de tenant regular, ele não será cobrado duas vezes.

### Controle de Acesso

Os usuários podem ser divididos em grupos. É para isso que serve o campo `groupIds`, que é opcional.

### @Menções

Por padrão, as `@menções` usarão `username` para buscar outros usuários SSO quando o caractere `@` for digitado. Se `displayName` for usado, os resultados correspondentes a `username` serão ignorados quando houver uma correspondência para `displayName`, e os resultados da busca de `@menções` usarão `displayName`.

### Assinaturas

Com o FastComments, os usuários podem assinar uma página clicando no ícone de sino no widget de comentários e clicando em Assinar.

Com um usuário regular, enviamos e-mails de notificação com base nas suas configurações de notificação.

Com Usuários SSO, separamos isso por compatibilidade com versões anteriores. Os usuários só receberão esses e-mails adicionais de notificação de assinatura se você definir `optedInSubscriptionNotifications` como `true`.

### Emblemas

Você pode atribuir emblemas a usuários SSO usando a propriedade `badgeConfig`. Emblemas são indicadores visuais que aparecem ao lado do nome de um usuário nos comentários.

- `badgeIds` - Um array de IDs de emblemas a serem atribuídos ao usuário. Estes são emblemas globais visíveis em todas as páginas. Devem ser IDs de emblemas válidos criados na sua conta FastComments. Limitado a 30 emblemas.
- `pageBadgeIds` - Um array opcional de IDs de emblemas com escopo na página atual (`urlId`). Esses emblemas são exibidos apenas na página onde foram atribuídos. Páginas diferentes podem ter emblemas com escopo de página diferentes para o mesmo usuário.
- `override` - Se true, todos os emblemas exibidos existentes serão substituídos pelos fornecidos. Emblemas globais e com escopo de página são substituídos de forma independente — substituir emblemas globais não afeta emblemas com escopo de página, e vice-versa. Se false ou omitido, os emblemas fornecidos serão adicionados aos emblemas existentes.
- `update` - Se true, as propriedades de exibição dos emblemas serão atualizadas a partir da configuração do tenant sempre que o usuário fizer login.