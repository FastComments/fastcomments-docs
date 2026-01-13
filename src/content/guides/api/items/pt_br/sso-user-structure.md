FastComments fornece uma solução SSO fácil de usar. Atualizar as informações de um usuário com a integração baseada em HMAC é tão simples quanto fazer o usuário carregar a página com um payload atualizado.

No entanto, pode ser desejável gerenciar um usuário fora desse fluxo, para melhorar a consistência da sua aplicação.

A SSO User API fornece uma maneira de fazer CRUD em objetos que chamamos de SSOUsers. Esses objetos são diferentes dos Users regulares e mantidos separados por segurança de tipos.

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
    isAccountOwner?: boolean // Permissão de administrador - Usuários SSO com essa flag são cobrados como SSO Admins (separados dos usuários SSO regulares)
    isAdminAdmin?: boolean // Permissão de administrador - Usuários SSO com essa flag são cobrados como SSO Admins (separados dos usuários SSO regulares)
    isCommentModeratorAdmin?: boolean // Permissão de moderador - Usuários SSO com essa flag são cobrados como SSO Moderators (separados dos usuários SSO regulares)
    /** Se null, o Controle de Acesso não será aplicado ao usuário. Se uma lista vazia, este usuário não poderá ver nenhuma página nem @mention outros usuários. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Não permita que outros usuários vejam a atividade deste usuário, incluindo comentários, em seu perfil. O padrão é true para fornecer perfis seguros por padrão. **/
    isProfileActivityPrivate?: boolean
    /** Não permita que outros usuários deixem comentários no perfil do usuário, ou vejam comentários de perfil existentes. Padrão false. **/
    isProfileCommentsPrivate?: boolean
    /** Não permita que outros usuários enviem mensagens diretas para este usuário. Padrão false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configuração opcional para insígnias do usuário. **/
    badgeConfig?: {
        /** Array de IDs de insígnias para atribuir ao usuário. Limitado a 30 insígnias. A ordem é respeitada. **/
        badgeIds: string[]
        /** Se true, substitui todas as insígnias exibidas existentes pelas fornecidas. Se false, adiciona às insígnias existentes. **/
        override?: boolean
        /** Se true, atualiza as propriedades de exibição das insígnias a partir da configuração do tenant. **/
        update?: boolean
    }
}
[inline-code-end]

### Cobrança para Usuários SSO

Usuários SSO são cobrados de forma diferente com base em suas flags de permissão:

- **Usuários SSO regulares**: Usuários sem permissões de administrador ou moderador são cobrados como usuários SSO regulares
- **SSO Admins**: Usuários com as flags `isAccountOwner` ou `isAdminAdmin` são cobrados separadamente como SSO Admins (mesma tarifa que administradores regulares do tenant)
- **SSO Moderators**: Usuários com a flag `isCommentModeratorAdmin` são cobrados separadamente como SSO Moderators (mesma tarifa que moderadores regulares)

**Importante**: Para evitar dupla cobrança, o sistema deduplica automaticamente usuários SSO em relação aos usuários e moderadores regulares do tenant pelo endereço de email. Se um usuário SSO tiver o mesmo email que um usuário ou moderador regular do tenant, ele não será cobrado duas vezes.

### Controle de Acesso

Usuários podem ser divididos em grupos. É para isso que serve o campo `groupIds`, e é opcional.

### @Menções

Por padrão `@mentions` usará `username` para buscar outros usuários SSO quando o caractere `@` for digitado. Se `displayName` for usado, então resultados que correspondam a `username` serão ignorados quando houver uma correspondência por `displayName`, e os resultados da busca de `@mention` usarão `displayName`.

### Assinaturas

Com o FastComments, os usuários podem se inscrever em uma página clicando no ícone de sino no widget de comentários e clicando em Assinar.

Com um usuário regular, nós enviamos emails de notificação com base nas configurações de notificação dele.

Com Usuários SSO, dividimos isso por compatibilidade com versões anteriores. Os usuários só receberão esses emails adicionais de notificação de assinatura se você definir `optedInSubscriptionNotifications` como `true`.

### Insígnias

Você pode atribuir insígnias a usuários SSO usando a propriedade `badgeConfig`. Insígnias são indicadores visuais que aparecem ao lado do nome do usuário nos comentários.

- `badgeIds` - Um array de IDs de insígnias para atribuir ao usuário. Estes devem ser IDs de insígnias válidos criados na sua conta FastComments. Limitado a 30 insígnias.
- `override` - Se true, substitui todas as insígnias existentes exibidas pelas fornecidas. Se false ou omitido, as insígnias fornecidas serão adicionadas às insígnias existentes.
- `update` - Se true, as propriedades de exibição das insígnias serão atualizadas a partir da configuração do tenant sempre que o usuário fizer login.