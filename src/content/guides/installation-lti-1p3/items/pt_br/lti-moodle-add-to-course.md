Este guia cobre como adicionar o FastComments a um curso Moodle 4.x depois que o administrador do site registrou a ferramenta e a configurou para aparecer no seletor de atividades. Se o FastComments ainda não estiver registrado, veja primeiro o guia de registro do Moodle.

#### Abrir o curso em modo de edição

1. Faça login no Moodle como um Editing Teacher (ou superior) para o curso.
2. Abra o curso.
3. Ative o **Edit mode** usando o interruptor no canto superior direito do cabeçalho do curso.

O Moodle 4.x substituiu o antigo menu suspenso "Add an activity or resource" usado no 3.x por uma caixa de diálogo de seleção de atividades em tela cheia. O Moodle 4.5 mantém esse seletor, mas adiciona uma linha de favoritos/estrela no topo, então marcar o FastComments uma vez facilita o acesso nas seções posteriores.

#### Adicionar a atividade FastComments

1. Role até a seção do curso (tópico ou semana) onde a discussão deve ficar.
2. Clique em **Add an activity or resource** na parte inferior dessa seção.
3. Na caixa de seleção, selecione **FastComments**. Se você não o vir, vá para a seção de problemas comuns abaixo.

O formulário de configurações da atividade abre. Os campos que importam:

- **Activity name** (obrigatório). Exibido na página do curso e no gradebook. Exemplo: `Week 3 Discussion`.
- **Activity description**. Texto introdutório opcional renderizado acima do fio de comentários.
- **Show description on course page**. Marque isto se quiser que a descrição fique visível sem precisar clicar na atividade.
- **Preconfigured tool**. Ajustado para `FastComments` (selecionado automaticamente quando iniciado a partir do seletor). Não altere.
- **Launch container**. Defina como **New window**. Veja a seção de problemas comuns para entender por que "Same window" pode quebrar em algumas implantações do Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Deixe em branco. O Dynamic Registration tratou desses valores no nível do site.

Role até o fim e clique em **Save and return to course** (ou **Save and display** para abrir a atividade imediatamente).

A atividade aparece como uma linha na seção com o ícone do FastComments. Os estudantes clicam na linha para abrir o fio de comentários.

#### Embutir o FastComments inline com o editor

Para um fio dentro de uma Page, capítulo do Book, Lesson ou qualquer outro recurso que use o editor Atto ou TinyMCE:

1. Abra o recurso em modo de edição.
2. Coloque o cursor onde o fio deve aparecer.
3. Na barra de ferramentas do editor, clique no botão **LTI** / **External tool**. No Atto é rotulado como "Insert LTI Advantage content". No TinyMCE (padrão no Moodle 4.3+) está no menu **More** como **External tools**.
4. Escolha **FastComments** na lista de ferramentas.
5. O FastComments abre um seletor de deep-linking. Confirme o título do fio e clique em **Embed**.
6. O editor insere um bloco de espaço reservado LTI. Salve o recurso.

Cada instância embutida é um fio distinto indexado pelo ID do item de conteúdo do deep-link, então uma Page com três embeds do FastComments gera três fios independentes.

#### Restrições de acesso e configurações de grupos

As configurações padrão de atividade do Moodle aplicam-se às atividades FastComments:

- **Common module settings** > **Group mode**. Definir isto para **Separate groups** ou **Visible groups** por si só não divide o FastComments em fios por grupo. O modo de grupo do Moodle apenas filtra o gradebook e a lista de membros. Para ter um fio separado por grupo, adicione uma atividade FastComments por grupo e use **Restrict access** para escopar cada uma.
- **Restrict access** > **Add restriction**. Suporta as condições padrão do Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, e conjuntos de restrição aninhados. Use **Group** para bloquear uma atividade FastComments a um único grupo.
- **Activity completion**. Configure para **Students must view this activity to complete it** se quiser rastreamento de conclusão. O FastComments atualmente não reporta um evento de conclusão de volta ao Moodle além do lançamento.

#### Mapeamento de papéis

O FastComments lê a reivindicação LTI `roles` que o Moodle envia em cada lançamento e mapeia da seguinte forma:

- Moodle **Manager** ou **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ou **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> somente leitura

Admins podem deletar qualquer comentário, banir usuários e editar configurações do fio. Moderadores podem deletar e aprovar comentários dentro do fio em que foram lançados. Papéis personalizados do Moodle herdam o mapeamento do arquétipo do qual foram clonados.

#### O que os estudantes veem

Os estudantes clicam na atividade FastComments (ou rolam até o bloco embutido dentro de uma Page ou Book). O Moodle envia a identidade deles ao FastComments via o lançamento LTI:

- Sem tela de login. O FastComments os autentica usando a conta do Moodle.
- O nome exibido, email e avatar vêm do Moodle.
- O fio é escopado para `(Moodle site, course, resource link ID)`, então a mesma atividade duplicada em outro curso recebe um fio novo.
- Respostas em árvore, votações e notificações funcionam do mesmo modo que em um fio standalone do FastComments.

#### Restringir o acesso público (recomendado)

Por padrão, os dados de comentários do FastComments são publicamente legíveis. Qualquer pessoa que conseguir adivinhar a URL do fio ou o endpoint da API pode ver os comentários, mesmo fora do Moodle. Para discussões de curso, você quase certamente vai querer restringir a visualização apenas a estudantes matriculados.

Abra sua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de customização do widget</a> e crie uma regra com **Require SSO To View Comments** habilitado, então defina o nível de segurança para **Secure SSO** para que os fios só possam ser carregados através do lançamento LTI assinado.

Veja [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para o walkthrough completo, incluindo como escopar a regra para um único domínio ou página.

#### Problemas comuns no Moodle

**FastComments ausente do seletor de atividades.** O administrador do site registrou a ferramenta mas não definiu **Tool configuration usage** para **Show in activity chooser and as a preconfigured tool**. Corrija isso em **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ícone de engrenagem no tile do FastComments.

**O lançamento falha ou mostra um quadro em branco quando definido como "Same window".** Os cookies de sessão do Moodle usam `SameSite=Lax` por padrão, e alguns navegadores os removem no POST cross-site que o LTI 1.3 usa para retornar do FastComments. Defina **Launch container** como **New window** na atividade. Isto é um requisito obrigatório para FastComments embutido dentro de uma Page ou Book, já que o caminho de lançamento embutido pelo editor sempre abre uma nova janela.

**A reivindicação `iss` é a URL do site Moodle, não um ID de tenant.** O FastComments usa a URL do site Moodle (o valor de configuração `wwwroot`) como o issuer LTI. Se sua instância Moodle mudar para um novo domínio ou você alterar `wwwroot`, os fios existentes do FastComments permanecerão vinculados ao issuer antigo e não corresponderão aos novos lançamentos. Reregistre a ferramenta contra a nova URL e migre os fios através do admin do FastComments se necessário.

**Backup e restauração de atividades.** Fazer backup de um curso e restaurá-lo em um curso novo cria novos resource link IDs, portanto as atividades FastComments restauradas começam com fios vazios. O curso original mantém os fios originais. Isso é comportamento intencional, não um bug.

**TinyMCE padrão no Moodle 4.5.** O Moodle 4.5 é distribuído com o TinyMCE como editor padrão para novas instalações. A localização do botão External tool está no menu **More** (`...`) em vez da barra principal. Sites mais antigos que fizeram upgrade a partir do 4.1 mantêm o Atto, a menos que um administrador tenha alterado o padrão.