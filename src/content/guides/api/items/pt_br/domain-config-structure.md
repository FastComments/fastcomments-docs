Um objeto `DomainConfig` representa a configuração de um domínio para um tenant.

A estrutura do objeto `DomainConfig` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do Domain Config'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Um domínio, não uma URL, como "fastcomments.com" ou "www.example.com". O subdomínio pode ser incluído se desejar limitar a um subdomínio. Máximo de 1000 caracteres. **/
    domain: string
    /** O Nome de Origem (From-Name) usado ao enviar e-mails. **/
    emailFromName?: string
    /** O E-mail de Origem (From-Email) usado ao enviar e-mails. Certifique-se de que o SPF está configurado para permitir que mail.fastcomments.com envie e-mails como o domínio usado neste atributo. **/
    emailFromEmail?: string
    /** READONLY (somente leitura). Quando o objeto foi criado. **/
    createdAt: string
    /** O logotipo relacionado a este domínio. Usado em e-mails. Use HTTPS. **/
    logoSrc?: string
    /** Um logotipo menor relacionado a este domínio. Use HTTPS. **/
    logoSrc100px?: string
    /** SOMENTE SSO. A URL usada no rodapé de cada e-mail enviado. Suporta uma variável "[userId]". **/
    footerUnsubscribeURL?: string
    /** SOMENTE SSO. Os cabeçalhos usados em cada e-mail enviado. Útil, por exemplo, para definir cabeçalhos relacionados a cancelamento de inscrição para melhorar a entrega. A entrada List-Unsubscribe neste Record, se existir, suporta uma variável "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Desativa todos os links de cancelamento de inscrição. Não recomendado, pode prejudicar as taxas de entrega. **/
    disableUnsubscribeLinks?: boolean
    /** Configuração DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura do DKIM Config'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** O nome de domínio no seu registro DKIM. **/
    domainName: string
    /** O seletor de chave DKIM a ser usado. **/
    keySelector: string
    /** A chave pública, em formato PEM. Retornada nas respostas GET. **/
    publicKey: string
    /** @deprecated Não é mais retornada nas respostas da API. Aceita na escrita para compatibilidade retroativa. **/
    privateKey?: string
}
[inline-code-end]

### Para Autenticação

A Configuração de Domínio é usada para determinar quais sites podem hospedar o widget do FastComments para sua conta. Esta é uma forma básica
de autenticação, o que significa que adicionar ou remover qualquer Configuração de Domínio pode impactar a disponibilidade da sua instalação do FastComments
em produção.

Não remova ou atualize a propriedade `domain` de um `Domain Config` para um domínio que está atualmente em uso, a menos que a intenção seja desativar esse domínio.

Isso tem o mesmo comportamento que remover um domínio de [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Observe também que remover um domínio da interface `My Domains` removerá qualquer configuração correspondente para esse domínio que possa ter sido adicionada através dessa interface.

### Para Personalização de E-mail

O link de cancelamento de inscrição no rodapé do e-mail e o recurso de cancelamento de inscrição com um clique oferecido por muitos clientes de e-mail podem ser configurados através desta API definindo `footerUnsubscribeURL` e `emailHeaders`, respectivamente.

### Para DKIM

Após definir seus registros DNS do DKIM, basta atualizar o DomainConfig com sua configuração DKIM usando a estrutura definida.