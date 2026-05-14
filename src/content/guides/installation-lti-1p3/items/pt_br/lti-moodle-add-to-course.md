Este guia cobre como adicionar o FastComments a um curso Moodle 4.x depois que um administrador do site registrou a ferramenta e a configurou para aparecer no seletor de atividades. Se o FastComments ainda não estiver registrado, veja primeiro o guia de registro do Moodle.

#### Open the Course in Edit Mode

1. Faça login no Moodle como um Editing Teacher (ou superior) para o curso.
2. Abra o curso.
3. Ative o **Edit mode** usando o interruptor no canto superior direito do cabeçalho do curso.

O Moodle 4.x substituiu o antigo dropdown "Add an activity or resource" usado no 3.x por um diálogo de escolha de atividade em tela cheia. O Moodle 4.5 mantém o mesmo seletor, mas adiciona uma linha de favoritos/estrela no topo, então fixar o FastComments uma vez facilita o acesso nas seções posteriores.

#### Add the FastComments Activity

1. Role até a seção do curso (tópico ou semana) onde a discussão pertence.
2. Clique em **Add an activity or resource** na parte inferior dessa seção.
3. No diálogo do seletor, selecione **FastComments**. Se você não o vir, pule para a seção de problemas conhecidos (gotchas) abaixo.

O formulário de configurações da atividade será aberto. Os campos importantes:

- **Activity name** (required). Aparece na página do curso e no gradebook. Exemplo: `Week 3 Discussion`.
- **Activity description**. Texto introdutório opcional renderizado acima do thread de comentários.
- **Show description on course page**. Marque se quiser que a descrição fique visível sem entrar na atividade.
- **Preconfigured tool**. Definido como `FastComments` (auto-selecionado quando iniciado pelo seletor). Não altere.
- **Launch container**. Defina como **New window**. Veja a seção de problemas conhecidos para entender por que "Same window" quebra em algumas implantações do Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Deixe em branco. O Registro Dinâmico (Dynamic Registration) tratou desses a nível do site.

Role até o final e clique em **Save and return to course** (ou **Save and display** para abrir a atividade imediatamente).

A atividade aparece como uma linha na seção com o ícone do FastComments. Os alunos clicam na linha para abrir o thread de comentários.

#### Embed FastComments Inline with the Editor

Para um thread dentro de uma Page, capítulo de Book, Lesson, ou qualquer outro recurso que use o editor Atto ou TinyMCE:

1. Abra o recurso em modo de edição.
2. Coloque o cursor onde o thread deve aparecer.
3. Na barra de ferramentas do editor, clique no botão **LTI** / **External tool**. No Atto está rotulado como "Insert LTI Advantage content". No TinyMCE (padrão no Moodle 4.3+) fica no menu **More** como **External tools**.
4. Escolha **FastComments** na lista de ferramentas.
5. O FastComments abre um seletor de deep-linking. Confirme o título do thread e clique em **Embed**.
6. O editor insere um bloco de espaço reservado LTI. Salve o recurso.

Cada instância embutida é um thread distinto identificado pelo deep-link content item ID, então uma Page com três embeds do FastComments terá três threads independentes.

#### Restrict Access and Group Settings

As configurações padrão de atividade do Moodle se aplicam às atividades FastComments:

- **Common module settings** > **Group mode**. Definir isso como **Separate groups** ou **Visible groups** não divide o FastComments em threads por grupo por si só. O modo de grupo do Moodle apenas filtra o gradebook e a lista de membros. Para executar um thread separado por grupo, adicione uma atividade FastComments por grupo e use **Restrict access** para limitar cada uma.
- **Restrict access** > **Add restriction**. Suporta as condições padrão do Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, e conjuntos de restrição aninhados. Use **Group** para bloquear uma atividade FastComments a um único grupo.
- **Activity completion**. Defina como **Students must view this activity to complete it** se você quiser rastreamento de conclusão. O FastComments atualmente não relata um evento de conclusão de volta ao Moodle além do lançamento.

#### Role Mapping

FastComments lê a claim `roles` do LTI que o Moodle envia em cada lançamento e mapeia da seguinte forma:

- Moodle **Manager** or **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** or **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> read-only

Admins podem excluir qualquer comentário, banir usuários e editar as configurações do thread. Moderators podem excluir e aprovar comentários dentro do thread em que foram lançados. Roles personalizados do Moodle herdam o mapeamento do arquétipo do qual foram clonados.

#### What Students See

Os alunos clicam na atividade FastComments (ou rolam até o bloco embutido dentro de uma Page ou Book). O Moodle envia a identidade deles para o FastComments via o lançamento LTI:

- Sem tela de login. O FastComments os autentica usando a conta do Moodle.
- O nome de exibição, e-mail e avatar vêm do Moodle.
- O thread é agrupado por `(Moodle site, course, resource link ID)`, então a mesma atividade duplicada em outro curso recebe um thread novo.
- Respostas em árvore, votação e notificações funcionam igual a um thread independente do FastComments.

#### Moodle Gotchas

**FastComments missing from the activity chooser.** O administrador do site registrou a ferramenta mas não definiu **Tool configuration usage** para **Show in activity chooser and as a preconfigured tool**. Corrija isso em **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ícone de engrenagem no tile do FastComments.

**Launch fails or shows a blank frame when set to "Same window".** Os cookies de sessão do Moodle usam `SameSite=Lax` por padrão, e alguns navegadores os removem no POST cross-site que o LTI 1.3 usa para retornar do FastComments. Defina **Launch container** para **New window** na atividade. Isso é um requisito firme para embeds do FastComments dentro de uma Page ou Book, já que o caminho de lançamento embutido no editor sempre abre uma nova janela.

**The `iss` claim is the Moodle site URL, not a tenant ID.** O FastComments usa a URL do site Moodle (o valor de configuração `wwwroot`) como o issuer do LTI. Se sua instância Moodle mudar para um novo domínio ou você alterar o `wwwroot`, threads existentes do FastComments permanecem vinculados ao issuer antigo e não corresponderão aos novos lançamentos. Re-registre a ferramenta contra a nova URL e migre threads através do admin do FastComments, se necessário.

**Activity backup and restore.** Fazer backup de um curso e restaurá-lo em um curso novo cria novos resource link IDs, então as atividades FastComments restauradas começam com threads vazios. O curso original mantém os threads originais. Isso é comportamento intencional, não um bug.

**Moodle 4.5 TinyMCE default.** O Moodle 4.5 é distribuído com o TinyMCE como editor padrão para instalações novas. A localização do botão External tool fica no menu **More** (`...`) em vez da barra principal. Sites mais antigos que atualizaram a partir do 4.1 mantêm o Atto, salvo se um administrador mudou o padrão.