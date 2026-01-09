Às vezes o FastComments precisa enviar e-mails para os seus usuários, especialmente se você não estiver usando Secure SSO.

Exemplos incluem verificar a conta ou atividade deles ao comentar pela primeira vez. O FastComments também enviará notificações sobre respostas aos comentários deles.

Quando o FastComments enviar e-mails para seus usuários, usaremos um Nome e um E-mail padrão de remetente: `FastComments Robot` e `noreply@fastcomments.com`.

Também usaremos nosso próprio logotipo no rodapé desses e-mails.

Se você tiver FastComments Flex ou Pro, tudo isso pode ser personalizado por domínio através da página "Meus Domínios":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Ao personalizar o logotipo exibido nos e-mails, verifique se o tamanho que você está enviando é o mesmo que deseja mostrar no rodapé do e-mail.

### Ao personalizar o `From Domain`

Se você personalizar o `From Domain`, provedores e clientes de e-mail precisam saber que o FastComments está autorizado a enviar e-mails em seu nome. Caso contrário, definir o `From Domain` e não seguir os passos abaixo provavelmente fará com que os e-mails caiam em spam.

#### 1. Configurar SPF

Para permitir que o FastComments envie e-mails de forma segura em nome do seu domínio, certifique-se de adicionar um registro SPF que nos autorize a fazê-lo.

Verifique se existem registros SPF que permitam que `mail.fastcomments.com` e `sib.fastcomments.com` enviem e-mails em nome do seu domínio.

Mais informações sobre como fazer isso estão aqui: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurar DKIM

Além do SPF, você deve configurar o DKIM. Quando a configuração do DNS estiver pronta, você pode clicar em "Mostrar avançado" na página de configurações de domínio para mostrar as configurações de DKIM por domínio.

Você também pode [invocar a API](/guide-api.html#domain-config-structure) para configurar o DKIM.

### Links de cancelamento de inscrição

Ao usar SSO, os recursos de cancelamento de inscrição usados em e-mails e notificações podem ser personalizados [via the DomainConfigs API](/guide-api.html#domain-config-structure).