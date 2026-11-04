SSO, ou single-sign-on (login único), é um conjunto de convenções usadas para permitir que você ou seus usuários usem o FastComments sem precisar criar outra conta.

Assumindo que você não permite comentários anônimos, uma conta é necessária para comentar com o FastComments. Tornamos esse processo de cadastro muito fácil - o usuário apenas deixa seu e-mail ao comentar.
No entanto, entendemos que mesmo isso é uma fricção extra que alguns sites desejam evitar.

Podemos reduzir essa fricção tendo apenas um fluxo de login para todo o seu site.

### Como obtenho isso?
Todos os tipos de conta atualmente têm acesso ao SSO. No entanto, o número máximo de usuários SSO varia de acordo com o seu pacote. Assim como outros recursos, os planos Pro e superiores oferecem suporte direto ao desenvolvimento.

Vamos comparar as opções e depois entrar nos detalhes de cada uma.

### Migração de Usuários e Comentários

Ao migrar de uma plataforma com SSO como o Disqus, você já terá os usuários e seus comentários.

Os comentários são importados como parte da sua migração, seja pela API, pela nossa UI de Importação ou pelo suporte ao cliente. A UI de Importação é preferível se ela suportar a plataforma da qual você
está migrando, pois incorpora tratamento de erros, extração e upload de avatares e mídias, e um sistema de monitoramento de tarefas em lote.

Os usuários em si são adicionados automaticamente ao visualizar as threads de comentários pela primeira vez. Alternativamente, eles podem ser pré-adicionados via API, mas esse trabalho não traz
muitas vantagens.

Se os comentários forem importados e os usuários SSO não forem adicionados manualmente via API, os comentários serão automaticamente migrados para a conta do usuário na primeira
vez que ela for criada quando ele visualizar qualquer thread de comentários. Eles então poderão gerenciar, editar e excluir os comentários que escreveram originalmente.

A migração automática é feita via e-mail ou nome de usuário. Algumas plataformas não fornecem e-mails na exportação, como o Disqus, então usamos o nome de usuário como fallback nesse caso.
- Contanto que você passe um nome de usuário correspondente e um e-mail no payload do SSO, adicionaremos o e-mail aos objetos de comentários individuais para que notificações e menções funcionem.

Se for desejado importar seus comentários e usuários de uma só vez, trabalhe com o suporte para migrar os comentários para as respectivas contas dos usuários após os usuários serem importados
via API.

Então, para resumir o caminho mais fácil para a migração:

1. Importe os comentários.
   1. Avatares e outras mídias são migrados automaticamente se você usar a UI de Importação em `Manage Data -> Imports`.
2. Configure o SSO Seguro ou Simples.
3. Deixe a migração acontecer por usuário automaticamente quando ele fizer login pela primeira vez.
   1. Isso geralmente adiciona menos de um segundo ao tempo de carregamento da página se o usuário tiver menos de 50 mil comentários.

### Usuários do WordPress
Se você está usando nosso <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">plugin do WordPress</a>, não há código para escrever! Basta ir à página Admin do plugin, clicar em SSO Settings e depois em Enable.

Isso levará você a um assistente de um único clique que criará sua chave de API, a enviará para a sua instalação do WordPress e ativará o SSO. Consolidamos isso em um único clique para você.

Observe que se você estiver instalando o plugin pela primeira vez, terá que concluir o processo de configuração antes de ver a página de administração com o botão SSO Settings.

#### SSO do WordPress - Moderadores

Observe que, atualmente, para que o selo "Moderator" apareça ao lado dos seus moderadores quando eles comentarem com o plugin do FastComments para WordPress,
eles também devem ser adicionados como Moderadores no painel do FastComments e ter seus e-mails verificados.

### Integrações Personalizadas

Para integrações personalizadas, existem duas opções.

### Opção Um - SSO Seguro

Com o SSO Seguro, o FastComments sabe que o usuário que comenta, vota e lê comentários é um usuário real do seu site.

Contanto que você crie um payload válido, o usuário sempre terá uma experiência de comentários sem interrupções.

Com o SSO Seguro, o payload do SSO é criado **no servidor** usando autenticação HMAC e depois passado para o widget no **cliente**.

Com o SSO Seguro, a conta do usuário é **completamente separada** do restante da base de usuários do FastComments. Isso significa que se tivermos dois parceiros,
Empresa A e Empresa B, cada um pode ter um usuário SSO com o nome de usuário "Bob".

#### Requisitos
- Conhecimento básico de desenvolvimento backend.
- Conhecimento básico sobre como lidar com chaves de API secretas.
- Conhecimento básico de desenvolvimento de API ou renderização no servidor.

#### Prós
- Seguro.
- Experiência de comentários sem interrupções.

#### Contras
- Requer desenvolvimento backend.

#### Atualizando Dados do Usuário

Com o SSO Seguro, cada vez que você passa o payload do usuário SSO, atualizamos o usuário dele com as informações mais recentes. Por exemplo, se
o usuário tem o nome de usuário `X` e você passa `Y` no payload do SSO, o nome de usuário dele se tornará `Y`.

Se você quiser remover valores usando essa abordagem, defina-os como `null` (não `undefined`).

#### API do SSO Seguro

Também fornecemos uma API para interagir com os usuários SSO. Veja [a documentação](/guide-api.html#sso-user-structure).

Observe que ao usar o SSO Seguro, os usuários são criados automaticamente nos bastidores no carregamento da página. Você não precisa importar seus usuários em massa.

### Opção Dois - SSO Simples

A alternativa ao SSO Seguro é simplesmente passar as informações do usuário para o widget de comentários.

Fornecer um e-mail com o SSO Simples não é obrigatório, porém sem isso os comentários deles aparecerão como "Unverified".

<sup>Nota!</sup> Desde o início de 2022, nomes de usuário com o SSO Simples não precisam ser únicos em todo o FastComments.com.

Idealmente, o SSO Simples deve ser escolhido apenas ao desenvolver em uma plataforma que não fornece acesso ao backend.

#### Requisitos
- Conhecimento básico de desenvolvimento client-side.
- É preciso saber pelo menos o e-mail do usuário.

#### Prós
- Simples.
- Toda atividade ainda é verificada.
- O usuário nunca insere seu nome de usuário ou e-mail.

#### Contras
- Menos seguro que o SSO Seguro, pois o payload do lado do cliente poderia ser manipulado para se tornar qualquer usuário.

#### API do SSO Simples

Os usuários criados automaticamente pelo fluxo do SSO Simples são armazenados como objetos `SSOUser`. Eles podem ser acessados e gerenciados via API `SSOUser`. Veja [a documentação](/guide-api.html#sso-user-structure).