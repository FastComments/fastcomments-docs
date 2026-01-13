Um objeto `DomainConfig` representa a configuração para um domínio de um locatário.

A estrutura para o objeto `DomainConfig` é a seguinte:

[inline-code-attrs-start title = 'Estrutura de Configuração de Domínio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Um domínio, não uma URL, como "fastcomments.com" ou "www.example.com". Subdomínio pode ser incluído se desejar limitar a um subdomínio. Máximo de 1000 caracteres. **/
    domain: string
    /** O nome do remetente (From-Name) usado ao enviar e-mails. **/
    emailFromName?: string
    /** O e-mail do remetente (From-Email) usado ao enviar e-mails. Garanta que o SPF esteja configurado para permitir que mail.fastcomments.com envie e-mails em nome do domínio usado neste atributo. **/
    emailFromEmail?: string
    /** APENAS LEITURA. Quando o objeto foi criado. **/
    createdAt: string
    /** O logo relacionado a este domínio. Usado em e-mails. Use HTTPS. **/
    logoSrc?: string
    /** Um logo menor relacionado a este domínio. Use HTTPS. **/
    logoSrc100px?: string
    /** APENAS SSO. A URL usada no rodapé de cada e-mail enviado. Suporta a variável "[userId]". **/
    footerUnsubscribeURL?: string
    /** APENAS SSO. Os cabeçalhos usados em cada e-mail enviado. Útil, por exemplo, para definir cabeçalhos relacionados a cancelamento de inscrição para melhorar a entrega. A entrada List-Unsubscribe neste Record, se existir, suporta a variável "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Desabilitar todos os links de cancelamento de inscrição. Não recomendado, pode prejudicar as taxas de entrega. **/
    disableUnsubscribeLinks?: boolean
    /** Configuração DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Configuração DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** O nome de domínio no seu registro DKIM. **/
    domainName: string
    /** O seletor de chave DKIM a ser usado. **/
    keySelector: string
    /** Sua chave privada. Comece com -----BEGIN PRIVATE KEY----- e termine com -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Para Autenticação

A Configuração de Domínio é usada para determinar quais sites podem hospedar o widget FastComments para sua conta. Esta é uma forma básica
de autenticação, o que significa que adicionar ou remover quaisquer Configurações de Domínio pode impactar a disponibilidade da sua instalação do FastComments
em produção.

Não remova ou atualize a propriedade `domain` de um `Domain Config` para um domínio que esteja atualmente em uso, a menos que a intenção seja desativar esse domínio.

Isso tem o mesmo comportamento que remover um domínio de [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Observe também que remover um domínio da interface `My Domains` removerá qualquer configuração correspondente para esse domínio que possa ter sido adicionada por meio desta interface.

### Para Personalização de E-mail

O link de cancelamento no rodapé do e-mail, e o recurso de cancelamento com um clique oferecido por muitos clientes de e-mail, podem ser configurados via esta API definindo `footerUnsubscribeURL` e `emailHeaders`, respectivamente.

### Para DKIM

Após definir seus registros DKIM no DNS, atualize simplesmente o DomainConfig com sua configuração DKIM usando a estrutura definida.

---