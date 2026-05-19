Esta página aborda como adicionar o FastComments a um curso Brightspace depois que um administrador registrou a ferramenta e criou uma implantação. Se a ferramenta ainda não estiver registrada, veja primeiro o guia de registro do D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments incorporado como um tópico de unidade no Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments executando dentro de uma unidade Brightspace, mostrando comentários em threads e um seletor de menção @-mention" />
</div>

O Brightspace oferece duas experiências de criação de conteúdo: **Classic Content** e a **New Content Experience** (também chamada **Lessons**). Ambas expõem o FastComments, mas os caminhos do menu diferem. Cada seção abaixo aborda ambos onde divergem.

#### Localizar a ferramenta FastComments

A ferramenta FastComments aparece em dois locais dentro do editor de conteúdo do curso:

1. O seletor de atividades, acessado a partir do botão **Add Existing** do módulo/unidade (rotulado **Add Existing Activities** em versões mais antigas do Brightspace). O FastComments aparece diretamente no seletor nas versões atuais do Brightspace; versões antigas o aninham sob um submenu **External Learning Tools**. Qualquer um dos caminhos adiciona o FastComments como um tópico independente.
2. O diálogo **Insert Stuff** dentro do editor HTML, em **LTI Advantage**. Isso incorpora o FastComments inline em um tópico HTML via o fluxo de deep linking do LTI.

Se o FastComments não aparecer em nenhum dos seletores, a implantação não está habilitada para a unidade organizacional que contém o curso. Peça ao administrador do Brightspace para abrir **Ferramentas do Administrador** > **Gerenciar Extensibilidade** > **LTI Advantage** > ferramenta FastComments > **Exibir Implantações**, abrir a implantação e adicionar a unidade organizacional do curso (ou uma unidade pai) em **Org Units**.

#### Adicionar o FastComments como um Tópico em um Módulo

Classic Content:

1. Abra o curso e clique em **Content** na barra de navegação.
2. Selecione o módulo que deve conter a discussão (ou crie um via **Add a module**).
3. Clique em **Add Existing** (Brightspace mais antigo: **Add Existing Activities** > **External Learning Tools**).
4. No seletor, clique em **FastComments**. O Brightspace cria um tópico no módulo e retorna à visualização de conteúdo.
5. Clique no novo tópico. Renomeie-o para algo descritivo como `FastComments Discussion` usando o editor de título inline.

New Content Experience (Lessons):

1. Abra o curso e clique em **Content**.
2. Abra a unidade e a lesson que devem conter a discussão.
3. Clique em **Add** > **Existing Activity** e selecione **FastComments** (Brightspace mais antigo: aninhado em **External Learning Tools**).
4. A atividade é adicionada à lesson.
5. Clique no título da atividade para renomeá-la.

Na primeira vez que qualquer usuário (instrutor ou estudante) abrir o tópico, o FastComments inicializa o thread para esse resource link. O thread está vinculado ao resource link ID, então renomear ou mover o tópico não altera qual thread é carregado.

#### Incorporar o FastComments Inline em um Tópico HTML

Use esse fluxo quando você quiser que os comentários apareçam abaixo de uma leitura, vídeo ou outro conteúdo dentro da mesma página do tópico em vez de como um tópico separado.

1. Abra ou crie um tópico HTML no módulo/lesson.
2. Clique em **Edit HTML** para abrir o editor HTML do Brightspace.
3. Coloque o cursor onde o thread de comentários deve aparecer.
4. Clique no botão **Insert Stuff** (ícone de peça de quebra-cabeça na barra de ferramentas do editor).
5. No diálogo Insert Stuff, role até **LTI Advantage** e clique em **FastComments**.
6. O FastComments abre um seletor de deep linking. Confirme a posição (as opções padrão funcionam para discussões de conteúdo); clique em **Insert** ou **Continue**.
7. O Brightspace retorna ao editor HTML com um bloco de substituição representando o lançamento LTI. Clique em **Save and Close** no tópico.

Quando o tópico for carregado, o Brightspace substitui o espaço reservado por um iframe que auto-inicia o FastComments via LTI. Os alunos veem o thread de discussão inline.

Um único tópico HTML pode conter múltiplas incorporações deep-linked do FastComments. Cada embed recebe seu próprio thread porque cada deep link produz um resource link ID distinto.

#### Tópico de Módulo vs Quicklink Inline

Escolha a abordagem de **tópico de módulo** quando:

- A discussão for a atividade principal daquela etapa no módulo.
- Você quiser que o tópico apareça no sumário do Brightspace, no rastreamento de conclusão e no Class Progress.

Escolha a abordagem de **embed inline** quando:

- Os comentários devem ficar abaixo de outro conteúdo na mesma página.
- Você não quer um item separado rastreável para conclusão no sumário.

#### Visibilidade, Rascunho e Condições de Liberação

Um novo tópico FastComments é visível para os alunos por padrão. Para ocultá-lo enquanto você o configura:

1. No editor de conteúdo, clique no título do tópico (Classic) ou no menu de três pontos da atividade (New Content Experience).
2. Defina o status para **Draft** (Classic) ou desative **Visibility** (New Content Experience).

Tópicos em rascunho são invisíveis para os alunos. Instrutores e TAs ainda os veem com um distintivo "Draft".

Para restringir o tópico a um grupo ou seção específica:

1. Abra o tópico.
2. Clique no menu do título do tópico > **Edit Properties In-place** (Classic) ou **Edit** > **Restrictions** (New Content Experience).
3. Em **Release Conditions**, clique em **Create**.
4. Escolha **Group enrollment** ou **Section enrollment**, selecione o grupo/seção e salve.

As condições de liberação se acumulam com o próprio mapeamento de funções do FastComments. Estudantes que não podem ver o tópico não recebem um LTI launch.

#### O que os Estudantes Vêm no Primeiro Lançamento

Quando um estudante clica no tópico (ou carrega um tópico HTML com um embed):

1. O Brightspace realiza o LTI 1.3 launch em segundo plano.
2. O FastComments recebe o nome do estudante, email, URL do avatar e função no LMS, e realiza o login automaticamente. Não há prompt de login do FastComments.
3. O thread de comentários para esse resource link é renderizado dentro do iframe do Brightspace.

Mapeamento de funções no lançamento:

- Brightspace `Administrator` becomes a FastComments **admin** for the thread (full moderation, delete, ban, and configuration access).
- Brightspace `Instructor` becomes a FastComments **moderator** (pin, hide, delete, ban).
- All other roles (`Learner`, `TeachingAssistant`, etc.) become standard commenters.

Os comentários são atribuídos à conta do Brightspace do estudante. Se o estudante editar seu nome ou avatar no Brightspace, o próximo LTI launch sincroniza a alteração.

#### Restringir o Acesso Público (Recomendado)

Por padrão, os dados de comentários do FastComments são publicamente legíveis. Qualquer pessoa que consiga adivinhar a URL de um thread ou endpoint da API pode ver seus comentários, mesmo fora do Brightspace. Para discussões de curso você quase certamente vai querer restringir a visualização apenas aos alunos matriculados.

Abra sua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de customização do widget</a> e crie uma regra com **Require SSO To View Comments** habilitado, então defina o nível de segurança para **Secure SSO** para que os threads só possam ser carregados através do lançamento LTI assinado.

Veja [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para o passo a passo completo, incluindo como aplicar a regra a um único domínio ou página.

#### Altura do Iframe e Redimensionamento

O FastComments emite a postMessage `org.imsglobal.lti.frameResize` em cada renderização de thread e em mudanças de conteúdo (novo comentário, expandir respostas). O Brightspace escuta essa mensagem e ajusta a altura do iframe para que o thread não seja cortado e não mostre uma barra de rolagem interna.

Se o iframe permanecer com uma altura fixa e curta:

- Confirme que o curso está carregado via HTTPS. O listener postMessage do Brightspace rejeita frames de conteúdo misto.
- Confirme que nenhuma extensão do navegador está bloqueando o canal postMessage.
- Para embeds inline em um tópico HTML, o HTML circundante não deve envolver o iframe em um contêiner de altura fixa. Remova qualquer `style="height: ..."` inline do elemento pai.

#### Especificidades do Brightspace

**Ferramenta não aparecendo no seletor Add Existing.** A implantação não está habilitada para a unidade organizacional deste curso. Um administrador precisa adicionar a unidade organizacional (ou uma pai) à lista **Org Units** da implantação. O registro da ferramenta por si só não é suficiente; a implantação define quais cursos veem a ferramenta.

**`deployment_id` mismatch on launch.** O FastComments TOFU-pins o primeiro `deployment_id` que vê para um registro. Se um administrador excluir a implantação original e criar uma nova, lançamentos a partir da nova implantação são rejeitados com um erro de incompatibilidade de implantação. A correção é re-registrar o FastComments (gere uma nova URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-o aqui</a>) e execute o Dynamic Registration novamente); o registro de configuração antigo é substituído.

**Tool launches but shows "Invalid LTI launch".** O curso está em uma estrutura de tenant/org diferente daquela coberta pela implantação, ou a implantação foi desativada após o registro. Verifique novamente **Ferramentas do Administrador** > **Gerenciar Extensibilidade** > **LTI Advantage** > FastComments > alternador **Enabled** e a lista de unidades organizacionais da implantação.

**Names and roles missing inside FastComments.** O Brightspace envia lançamentos LTI com claims do Names and Role Provisioning Services (NRPS). Se um curso foi atualizado a partir de um link LTI 1.1 mais antigo, o lançamento pode não conter as claims `name` e `email`. Re-adicione o tópico FastComments via **Add Existing** (não migre o link antigo) para que o lançamento use LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** O tópico HTML foi inserido como um `<iframe>` simples apontando para o FastComments em vez de via **Insert Stuff** > **LTI Advantage**. Iframes simples pulam o LTI launch e levam os usuários para a página pública do FastComments. Exclua o iframe e reinsira via o fluxo Insert Stuff.