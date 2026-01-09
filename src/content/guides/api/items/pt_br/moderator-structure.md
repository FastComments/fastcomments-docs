Um objeto `Moderator` representa a configuração para um moderador.

Existem três tipos de moderadores:

1. Usuários administradores que possuem a flag `isCommentModeratorAdmin`.
2. Usuários SSO com a flag `isCommentModeratorAdmin`.
3. Comentadores regulares, ou usuários do FastComments.com, que são convidados como Moderadores.

A estrutura `Moderator` é usada para representar o Estado de Moderação do caso de uso `3`.

Se você quiser convidar um usuário para ser moderador via API, use a API `Moderator` criando um `Moderator` e `inviting` o usuário.

Se o usuário não tiver uma conta no FastComments.com, o e-mail de convite ajudará a configurá-la. Se já tiverem uma conta, eles receberão acesso de moderação ao seu tenant e o `userId` do objeto `Moderator` será atualizado para apontar para o usuário deles. Você não terá acesso via API ao usuário deles, pois nesse caso a conta pertence a eles e é gerenciada pelo FastComments.com.

Se você precisar de gerenciamento completo da conta do usuário, recomendamos usar SSO ou adicioná-los como um [Tenant User](https://fastcomments.com/auth/my-account/users) e então adicionar um objeto `Moderator` para rastrear suas estatísticas.

A estrutura `Moderator` pode ser usada como um mecanismo de rastreamento de estatísticas para os casos de uso `1` e `2`. Após criar o usuário, adicione um objeto `Moderator` com o `userId` dele definido e suas estatísticas serão rastreadas na [Página de Moderadores de Comentários](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

A estrutura do objeto `Moderator` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]