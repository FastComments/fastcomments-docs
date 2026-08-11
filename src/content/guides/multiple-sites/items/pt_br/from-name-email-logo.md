Às vezes, o FastComments precisa enviar e‑mail aos seus usuários, especialmente se você não estiver usando SSO seguro.

Exemplos disso incluem a verificação da conta ou da atividade deles ao comentar pela primeira vez. O FastComments também enviará notificações de respostas aos comentários deles.

Quando o FastComments envia e‑mail aos seus usuários, usaremos um Nome e E‑mail padrão de `FastComments Robot` e `noreply@fastcomments.com`.

Também usaremos nosso próprio logotipo no rodapé desses e‑mails.

Se você tem FastComments Flex ou Pro, tudo isso pode ser personalizado por domínio na página "Meus Domínios":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Formulário de configurações de e‑mail por domínio com os campos Nome do Remetente, E‑mail do Remetente e upload de logotipo'; title='Personalizando Nome do Remetente, E‑mail e Logotipo' app-screenshot-end]

Ao personalizar o logotipo exibido nos e‑mails, certifique‑se de que o tamanho que você está enviando seja o mesmo tamanho que deseja exibir no rodapé do e‑mail.

### Ao Personalizar o `From Domain`

Se você personalizar o `From Domain`, provedores de e‑mail e clientes precisam saber que o FastComments está autorizado a enviar e‑mails em seu nome. Caso contrário, definir o `From Domain` e não seguir os passos abaixo provavelmente fará com que os e‑mails vão para a pasta de spam.

#### 1. Configurar SPF

Para permitir que o FastComments envie e‑mail com segurança como seu domínio, certifique‑se de adicionar um registro SPF que nos autorize a fazê‑lo.

Certifique‑se de que existam registros SPF que permitam que `mail.fastcomments.com` e `sib.fastcomments.com` enviem e‑mail como seu domínio.

Mais informações sobre como fazer isso estão aqui: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurar DKIM

Além do SPF, você deve configurar DKIM. Quando sua configuração DNS estiver pronta, você pode clicar em "Show Advanced" na página de configurações de domínio para exibir as configurações DKIM por domínio.

Você também pode [invocar a API](/guide-api.html#domain-config-structure) para definir a configuração DKIM.

### Links de Cancelamento de Inscrição

Ao usar SSO, os recursos de cancelamento de inscrição usados em e‑mails e notificações podem ser personalizados [via a API DomainConfigs](/guide-api.html#domain-config-structure).

### Ofuscação de Links de E‑mail

Se a reputação do domínio do seu site está fazendo com que os e‑mails de notificação caiam no spam, você pode direcionar os botões "ver comentário" através de `fastcomments.com` em vez de vinculá‑los diretamente à sua página. Os provedores de caixa de correio avaliam cada link no corpo do e‑mail com base na reputação do destino, portanto, quando seu domínio está sendo sinalizado, os links diretos contribuem para a pontuação de spam independentemente de quão limpa esteja sua configuração de envio.

Ative isso em "Show Advanced" na página Meus Domínios, na seção "Email Link Obfuscation". A configuração é por domínio.

Quando ativado, os links em e‑mails de menção, resposta, novo comentário, página assinada, comentário de perfil e resumo são reescritos para tokens curtos que redirecionam para a página original ao serem clicados. O destino está vinculado ao seu locatário: o redirecionamento só encaminha URLs cujo host corresponda a um dos seus domínios configurados, e os tokens expiram automaticamente após 30 dias.

A experiência ao clicar permanece inalterada. Os leitores ainda chegam à sua página com o comentário já rolado para a visualização.