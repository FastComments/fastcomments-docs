`User` é um objeto que representa o denominador mais comum entre todos os usuários.

Lembre-se que na FastComments temos diversos casos de uso diferentes para usuários:

- Secure SSO
- Simple SSO
- Tenant Users (For example: Administrators)
- Commenters

Esta API é para **Commenters** e usuários criados via **Simple SSO**. Basicamente, qualquer usuário criado através do seu site pode ser acessado por esta API. Tenant Users também podem ser obtidos desta forma, mas você obterá mais informações ao interagir com a API `/tenant-users/`.

Para `Secure SSO` por favor use a API `/sso-users/`.

Você não pode atualizar esses tipos de usuários. Eles criaram sua conta através do seu site, então fornecemos um acesso básico somente leitura, mas você não pode fazer alterações. Se você quiser ter esse tipo de fluxo - você precisa configurar `Secure SSO`.

A estrutura do objeto `User` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Este também é o id usado como userId em objetos de comentário. **/
    id: string
    username: string
    /** Um link para o blog do comentarista, por exemplo. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]

---