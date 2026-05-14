Esta página aborda como adicionar o FastComments a um curso Brightspace depois que um administrador registrou a ferramenta e criou um deployment. Se a ferramenta ainda não estiver registrada, veja primeiro o guia de registro do D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments embedded as a unit topic in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments executando dentro de uma unidade do Brightspace, mostrando comentários encadeados e um seletor de menções com @-sinal" />
</div>

O Brightspace fornece duas experiências de autoria de conteúdo: **Classic Content** e a **New Content Experience** (também chamada **Lessons**). Ambas expõem o FastComments, mas os caminhos de menu diferem. Cada seção abaixo cobre ambos onde divergirem.

#### Localize a ferramenta FastComments

A ferramenta FastComments aparece em dois lugares dentro do editor de conteúdo do curso:

1. O seletor de atividades, acessado a partir do botão **Add Existing** do módulo/unidade (rotulado **Add Existing Activities** em versões mais antigas do Brightspace). O FastComments aparece diretamente no seletor em builds atuais do Brightspace; versões antigas o aninham sob um submenu **External Learning Tools**. Qualquer um dos caminhos adiciona o FastComments como um tópico independente.
2. A caixa de diálogo **Insert Stuff** dentro do editor HTML, em **LTI Advantage**. Isso embute o FastComments inline em um tópico HTML via o fluxo de deep linking do LTI.

Se o FastComments não aparecer em nenhum dos seletores, o deployment não está habilitado para a unidade organizacional (org unit) que contém o curso. Peça ao administrador do Brightspace para abrir **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, abrir o deployment e adicionar a org unit do curso (ou uma org unit pai) em **Org Units**.

#### Adicionar o FastComments como um tópico em um módulo

Classic Content:

1. Abra o curso e clique em **Content** na barra de navegação.
2. Selecione o módulo que deve conter a discussão (ou crie um via **Add a module**).
3. Clique em **Add Existing** (Brightspace mais antigo: **Add Existing Activities** > **External Learning Tools**).
4. No seletor, clique em **FastComments**. O Brightspace cria um tópico no módulo e retorna você à visualização de conteúdo.
5. Clique no novo tópico. Renomeie-o para algo descritivo como `FastComments Discussion` usando o editor de título inline.

New Content Experience (Lessons):

1. Abra o curso e clique em **Content**.
2. Abra a unidade e a lesson que devem conter a discussão.
3. Clique em **Add** > **Existing Activity** e selecione **FastComments** (Brightspace mais antigo: aninhado sob **External Learning Tools**).
4. A atividade é adicionada à lesson.
5. Clique no título da atividade para renomeá-la.

Na primeira vez que qualquer usuário (instrutor ou estudante) abrir o tópico, o FastComments inicializa o thread para esse resource link. O thread está vinculado ao resource link ID, então renomear ou mover o tópico não altera qual thread é carregado.

#### Embutir o FastComments inline em um tópico HTML

Use este fluxo quando você quiser que os comentários apareçam abaixo de uma leitura, vídeo ou outro conteúdo dentro da mesma página do tópico em vez de como um tópico separado.

1. Abra ou crie um tópico HTML no módulo/lesson.
2. Clique em **Edit HTML** para abrir o editor HTML do Brightspace.
3. Coloque o cursor onde o thread de comentários deve aparecer.
4. Clique no botão **Insert Stuff** (ícone de peça de quebra-cabeça na barra de ferramentas do editor).
5. Na caixa Insert Stuff, role até **LTI Advantage** e clique em **FastComments**.
6. O FastComments abre um seletor de deep linking. Confirme o posicionamento (as opções padrão funcionam para discussões de conteúdo); clique em **Insert** ou **Continue**.
7. O Brightspace retorna ao editor HTML com um bloco placeholder representando o launch LTI. Clique em **Save and Close** no tópico.

Quando o tópico for carregado, o Brightspace substitui o placeholder por um iframe que auto-inicia o FastComments via LTI. Os estudantes veem o thread de discussão inline.

Um único tópico HTML pode conter múltiplos embeds deep-linked do FastComments. Cada embed recebe seu próprio thread porque cada deep link produz um resource link ID distinto.

#### Tópico do módulo vs Quicklink inline

Escolha a abordagem de **module topic** quando:

- A discussão for a atividade principal para aquela etapa no módulo.
- Você quiser que o tópico apareça no sumário do Brightspace, no acompanhamento de conclusão e no Class Progress.

Escolha a abordagem de **inline embed** quando:

- Os comentários devem ficar abaixo de outro conteúdo na mesma página.
- Você não quer um item separado rastreável por conclusão no sumário.

#### Visibilidade, rascunho e condições de liberação

Um novo tópico FastComments é visível para os estudantes por padrão. Para escondê-lo enquanto você o configura:

1. No editor de conteúdo, clique no título do tópico (Classic) ou no menu de três pontos na atividade (New Content Experience).
2. Defina o status para **Draft** (Classic) ou desative a **Visibility** (New Content Experience).

Tópicos em Draft são invisíveis para estudantes. Instrutores e TAs ainda os veem com um badge "Draft".

Para restringir o tópico a um grupo ou seção específica:

1. Abra o tópico.
2. Clique no menu do título do tópico > **Edit Properties In-place** (Classic) ou **Edit** > **Restrictions** (New Content Experience).
3. Em **Release Conditions**, clique em **Create**.
4. Escolha **Group enrollment** ou **Section enrollment**, selecione o grupo/seção e salve.

As release conditions se acumulam com o próprio mapeamento de papéis do FastComments. Estudantes que não podem ver o tópico não recebem um lançamento LTI.

#### O que os estudantes veem no primeiro lançamento

Quando um estudante clica no tópico (ou carrega um tópico HTML com um embed):

1. O Brightspace realiza o launch LTI 1.3 em segundo plano.
2. O FastComments recebe o nome do estudante, email, URL do avatar e papel no LMS, e faz o login automaticamente. Não há prompt de login do FastComments.
3. O thread de comentários para esse resource link é renderizado dentro do iframe do Brightspace.

Mapeamento de papéis no lançamento:

- Brightspace `Administrator` se torna um FastComments **admin** para o thread (moderação completa, excluir, banir e acesso à configuração).
- Brightspace `Instructor` se torna um FastComments **moderator** (pin, ocultar, excluir, banir).
- Todos os outros papéis (`Learner`, `TeachingAssistant`, etc.) se tornam comentaristas padrão.

Os comentários são atribuídos à conta do estudante no Brightspace. Se o estudante editar seu nome ou avatar no Brightspace, o próximo lançamento LTI sincroniza a alteração.

#### Altura do iframe e redimensionamento

O FastComments emite a postMessage `org.imsglobal.lti.frameResize` em toda renderização de thread e em alterações de conteúdo (novo comentário, expandir respostas). O Brightspace escuta essa mensagem e ajusta a altura do iframe para que o thread não seja cortado e não mostre uma barra de rolagem interna.

Se o iframe permanecer com altura fixa e curta:

- Confirme que o curso está sendo carregado via HTTPS. O listener postMessage do Brightspace rejeita frames de conteúdo misto.
- Confirme que nenhuma extensão do navegador está bloqueando o canal postMessage.
- Para embeds inline em um tópico HTML, o HTML circundante não deve envolver o iframe em um contêiner de altura fixa. Remova qualquer `style="height: ..."` inline do elemento pai.

#### Particularidades específicas do Brightspace

**Ferramenta não aparecendo no seletor Add Existing.** O deployment não está habilitado para a org unit deste curso. Um administrador precisa adicionar a org unit (ou uma org unit pai) à lista **Org Units** do deployment. O registro da ferramenta por si só não é suficiente; o deployment define quais cursos veem a ferramenta.

**`deployment_id` em desacordo no lançamento.** O FastComments TOFU-pins o primeiro `deployment_id` que encontra para um registration. Se um administrador excluir o deployment original e criar um novo, lançamentos a partir do novo deployment são rejeitados com um erro de mismatch de deployment. A correção é re-registrar o FastComments (gerar uma nova URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-a aqui</a>) e executar o Dynamic Registration novamente); o registro de configuração antigo é substituído.

**A ferramenta inicia mas mostra "Invalid LTI launch".** O curso está em uma estrutura de tenant/org diferente daquela coberta pelo deployment, ou o deployment foi desabilitado após o registro. Verifique novamente **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > **FastComments** > alternador **Enabled** e a lista de org units do deployment.

**Nomes e papéis ausentes dentro do FastComments.** O Brightspace envia lançamentos LTI com claims do Names and Role Provisioning Services (NRPS). Se um curso foi atualizado a partir de um link LTI 1.1 mais antigo, o lançamento pode não conter as claims `name` e `email`. Re-adicione o tópico FastComments via **Add Existing** (não migre o link antigo) para que o lançamento use LTI 1.3.

**Embed mostra uma tela de login em vez de auto-SSO.** O tópico HTML foi inserido como um `<iframe>` simples apontando para o FastComments em vez de via **Insert Stuff** > **LTI Advantage**. Iframes simples pulam o lançamento LTI e direcionam os usuários para a página pública do FastComments. Delete o iframe e reinsira via o fluxo Insert Stuff.