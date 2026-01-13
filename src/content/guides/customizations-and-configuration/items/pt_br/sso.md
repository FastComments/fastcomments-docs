SSO, ou autenticação única (single-sign-on), é um conjunto de convenções utilizado para permitir que você ou seus usuários usem o FastComments sem precisar criar outra conta.

Assumindo que você não permita comentários anônimos, uma conta é necessária para comentar no FastComments. Tornamos esse processo de cadastro muito fácil — o usuário apenas deixa seu e-mail ao comentar.
Entendemos, porém, que mesmo isso é uma fricção adicional que alguns sites querem evitar.

Podemos reduzir essa fricção ao ter apenas um fluxo de login para todo o seu site.

### How do I get it?
Todos os tipos de conta atualmente têm acesso ao SSO. Entretanto, o número máximo de usuários SSO varia de acordo com seu pacote. Como em outros recursos, os planos Pro e superiores oferecem suporte de desenvolvimento direto.

Vamos comparar as opções e, em seguida, detalhar cada uma.

### User and Comment Migrations

Ao migrar de uma plataforma com SSO como o Disqus, você já terá os usuários e seus comentários.

Os comentários são importados como parte da sua migração, seja pela API, pela nossa Import UI, ou pelo suporte ao cliente. A Import UI é preferida se suportar a plataforma da qual você está migrando, pois ela incorpora tratamento de erros, extração e upload de avatares e mídias, e um sistema de monitoramento de jobs em lote.

Os próprios usuários são adicionados automaticamente ao visualizarem os tópicos de comentários pela primeira vez. Alternativamente, eles podem ser pré-adicionados via API, mas esse trabalho não traz muitas vantagens.

Se os comentários forem importados e os usuários SSO não forem adicionados manualmente via API, então os comentários serão automaticamente migrados para a conta do usuário na primeira vez que ela for criada quando visualizarem qualquer tópico de comentário. Eles então poderão gerenciar, editar e excluir os comentários que originalmente escreveram.

A migração automática é feita por e-mail ou nome de usuário. Algumas plataformas não fornecem e-mails na exportação, como o Disqus, então nesse caso usamos o nome de usuário.
- Desde que você passe um nome de usuário correspondente e um e-mail no payload do SSO, adicionaremos o e-mail aos objetos de comentário individuais para que notificações e menções funcionem.

Se desejar importar seus comentários e usuários de uma vez, trabalhe com o suporte para migrar os comentários para as respectivas contas dos usuários após os usuários serem importados
via a API.

So to summarize the easiest path to take for the migration is:

1. Import comments.
   1. Avatares e outras mídias são migrados automaticamente se usar a Import UI em `Manage Data -> Imports`.
2. Setup Secure or Simple SSO.
3. Deixe a migração ocorrer por usuário automaticamente quando eles fizerem login pela primeira vez.
   1. Isso geralmente adiciona menos de um segundo ao tempo de carregamento da página se o usuário tiver menos de 50k comentários.

### WordPress Users
Se você estiver usando nosso <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">plugin do WordPress</a>, não há código para escrever! Simplesmente vá para a página de administração do plugin, clique em Configurações de SSO e então Ativar.

Isso o levará a um assistente de um único botão que criará sua chave de API, a enviará para sua instalação do WordPress e ativará o SSO. Consolidamos isso em um único clique para você.

Observe que se você estiver instalando o plugin pela primeira vez, terá que seguir o processo de configuração antes de ver a página de administração com o botão Configurações de SSO.

#### WordPress SSO - Moderators

Observe que atualmente, para que a insígnia "Moderator" apareça ao lado dos seus moderadores quando eles comentarem com o plugin FastComments para WordPress, eles também devem ser adicionados como Moderadores no painel do FastComments e ter o e-mail verificado.

### Custom Integrations

Para integrações personalizadas, há duas opções.

### Option One - Secure SSO

Com o Secure SSO, o FastComments sabe que o usuário que comenta, vota e lê comentários é um usuário real do seu site.

Contanto que você crie um payload válido, o usuário sempre terá uma experiência de comentários sem interrupções.

Com o Secure SSO, o payload do SSO é criado **no lado do servidor** usando autenticação HMAC e então passado para o widget no **cliente**.

Com o Secure SSO, a conta do usuário é **completamente separada** do restante da base de usuários do FastComments. Isso significa que, se tivermos dois parceiros Empresa A e Empresa B, cada um pode ter um usuário SSO com o nome de usuário "Bob".

#### Requirements
- Algum conhecimento básico de desenvolvimento backend.
- Algum conhecimento básico sobre como lidar com chaves de API secretas.
- Algum conhecimento básico sobre desenvolvimento de APIs ou renderização no lado do servidor.

#### Pros
- Seguro.
- Experiência de comentários sem interrupções.

#### Cons
- Requer desenvolvimento backend.

#### Updating User Data

Com o Secure SSO, cada vez que você enviar o payload de usuário SSO, atualizaremos o usuário com as informações mais recentes. Por exemplo, se o usuário tem o nome de usuário `X`, e você enviar `Y` no payload do SSO, o nome de usuário dele se tornará `Y`.

Se você quiser remover valores usando essa abordagem, defina-os como `null` (não `undefined`).

#### Secure SSO API

Também fornecemos uma API para interagir com os usuários SSO. Veja [a documentação](/guide-api.html#sso-user-structure).

Observe que, ao usar o Secure SSO, os usuários são criados automaticamente nos bastidores ao carregar a página. Você não precisa importar seus usuários em massa.

### Option Two - Simple SSO

A alternativa ao Secure SSO é simplesmente passar as informações do usuário para o widget de comentários.

Fornecer um e-mail com o Simple SSO não é obrigatório; no entanto, sem isso os comentários aparecerão como "Não verificado".

<sup>Observação!</sup> A partir do início de 2022, nomes de usuário com Simple SSO não precisam ser únicos em todo o FastComments.com.

Idealmente, o Simple SSO deve ser escolhido apenas quando se desenvolve em uma plataforma que não fornece acesso ao backend.

#### Requirements
- Algum conhecimento básico de desenvolvimento do lado do cliente.
- É necessário conhecer pelo menos o e-mail do usuário.

#### Pros
- Simples.
- Toda a atividade ainda é verificada.
- O usuário nunca precisa inserir seu nome de usuário ou e-mail.

#### Cons
- Menos seguro que o Secure SSO, já que o payload do lado do cliente poderia ser forjado para se tornar qualquer usuário.

#### Simple SSO API

Usuários criados automaticamente via o fluxo Simple SSO são armazenados como objetos `SSOUser`. Eles podem ser acessados e gerenciados via a API `SSOUser`. Veja [a documentação](/guide-api.html#sso-user-structure).

---