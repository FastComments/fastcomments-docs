Às vezes o FastComments precisa enviar e-mails para seus usuários, especialmente se você não estiver usando SSO seguro.

Exemplos disso incluem verificar a conta deles ou a atividade ao comentar pela primeira vez. O FastComments
também enviará notificações para respostas aos comentários deles.

Quando o FastComments envia e-mails para seus usuários, usaremos um Nome e E-mail padrão de From de `FastComments Robot` e `noreply@fastcomments.com`.

Também usaremos nosso próprio logo no rodapé desses e-mails.

Se você tiver FastComments Flex ou Pro, tudo isso pode ser personalizado por domínio através da "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Ao personalizar o logo exibido nos e-mails, verifique se o tamanho que você está carregando é o mesmo que deseja mostrar no rodapé do e-mail.

### When Customizing The `From Domain`

Se você personalizar o `From Domain`, provedores e clientes de e-mail precisam saber que o FastComments está autorizado a enviar e-mails em seu nome. Caso contrário,
definir o `From Domain` e não seguir os passos abaixo provavelmente fará com que os e-mails caiam como spam.

#### 1. Setup SPF

Para permitir que o FastComments envie e-mails com segurança em nome do seu domínio, certifique-se de adicionar um registro SPF que nos permita fazer isso.

Certifique-se de que existam registros SPF que permitam que `mail.fastcomments.com` e `sib.fastcomments.com` enviem e-mail em nome do seu domínio.

Mais informações sobre como fazer isso estão aqui: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

Além do SPF, você deve configurar o DKIM. Depois que a configuração do DNS estiver pronta, você pode clicar em "Mostrar Avançado" na página de configurações de domínio
para exibir as configurações de DKIM por domínio.

Você também pode [usar a API](/guide-api.html#domain-config-structure) para definir a configuração de DKIM.

### Unsubscribe Links

Ao usar SSO, os recursos de cancelamento de inscrição usados em e-mails e notificações podem ser personalizados [por meio da DomainConfigs API](/guide-api.html#domain-config-structure).

### Email Link Obfuscation

Se a reputação do domínio do seu site estiver fazendo com que os e-mails de notificação caiam em spam, você pode rotear os botões "view comment" através de `fastcomments.com` em vez de vincular diretamente à sua página. Os provedores de caixa de correio avaliam cada link no corpo do e-mail em relação à reputação do destino, então quando seu domínio está sendo sinalizado, os links diretos contribuem para a pontuação de spam independentemente de quão adequada seja sua configuração de envio.

Ative isso em "Mostrar Avançado" na página My Domains, na seção "Email Link Obfuscation". A configuração é por domínio.

Quando ativado, os links em e-mails mention, reply, new-comment, subscribed-page, profile-comment, e digest são reescritos para tokens curtos que redirecionam para a página original ao serem clicados. O destino está vinculado ao seu tenant: o redirecionamento só encaminha para URLs cujo host corresponda a um dos seus domínios configurados, e os tokens expiram automaticamente após 30 dias.

A experiência após o clique permanece inalterada. Os leitores ainda chegam à sua página com o comentário deslocado para visualização.