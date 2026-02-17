Um `DomainConfig` object representa a configuração para um domínio de um tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Estrutura de DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Um domínio, não uma URL, como "fastcomments.com" ou "www.example.com". Subdomínio pode ser incluído se desejar limitar a um subdomínio. Máximo de 1000 caracteres. **/
    domain: string
    /** O From-Name usado ao enviar e-mails. **/
    emailFromName?: string
    /** O From-Email usado ao enviar e-mails. Certifique-se de que o SPF está configurado para permitir que mail.fastcomments.com envie e-mails como o domínio usado neste atributo. **/
    emailFromEmail?: string
    /** SOMENTE LEITURA. Quando o objeto foi criado. **/
    createdAt: string
    /** O logotipo relacionado a este domínio. Usado em e-mails. Use HTTPS. **/
    logoSrc?: string
    /** Um logotipo menor relacionado a este domínio. Use HTTPS. **/
    logoSrc100px?: string
    /** SOMENTE SSO. A URL usada no rodapé de cada e-mail enviado. Suporta a variável "[userId]". **/
    footerUnsubscribeURL?: string
    /** SOMENTE SSO. Os cabeçalhos usados em cada e-mail enviado. Útil, por exemplo, para definir cabeçalhos relacionados a cancelamento de inscrição para melhorar a entrega. A entrada List-Unsubscribe neste Record, se existir, suporta a variável "[userId]". **/
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
    /** A chave pública, em formato PEM. Retornada em respostas GET. **/
    publicKey: string
    /** @deprecated Não é mais retornada em respostas da API. Aceita na escrita por compatibilidade retroativa. **/
    privateKey?: string
}
[inline-code-end]

### Para Autenticação

Domain Configuration é usado para determinar quais sites podem hospedar o widget FastComments para sua conta. Esta é uma forma básica
de autenticação, o que significa que adicionar ou remover qualquer Domain Configuration pode impactar a disponibilidade da sua instalação FastComments
em produção.

Não remova ou atualize a propriedade `domain` de um `Domain Config` para um domínio que esteja atualmente em uso, a menos que a intenção seja desativar esse domínio.

This has the same behavior as removing a domain from [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Observe também que remover um domínio da interface `My Domains` removerá qualquer configuração correspondente para esse domínio que tenha sido adicionada por meio desta interface.

### Para Personalização de E-mails

O link de cancelamento de inscrição no rodapé do e-mail, e o recurso de cancelamento com um clique oferecido por muitos clientes de e-mail, podem ser configurados por esta API definindo `footerUnsubscribeURL` e `emailHeaders`, respectivamente.

### Para DKIM

Após definir seus registros DNS DKIM, simplesmente atualize o DomainConfig com sua configuração DKIM usando a estrutura definida. 

---