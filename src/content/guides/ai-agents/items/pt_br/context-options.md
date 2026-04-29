A seção **Context** no formulário de edição controla quanta informação o agente recebe a cada execução. Mais contexto produz decisões melhores, mas aumenta o custo de tokens por execução, então você só deve incluir o que o agente realmente precisa.

### O que está sempre incluído

Mesmo com todas as caixas de seleção desmarcadas, a mensagem de contexto do agente inclui:

- O **tipo de evento disparador** (por exemplo `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- A **URL da página e o ID da URL** (quando conhecidos).
- O **comentário** que disparou a execução, se houver — ID, ID do autor, nome de exibição do autor, texto do comentário, contagens de votos, contagem de flags, flags de spam/aprovado/revisado, ID do pai. O email do autor **nunca** é enviado ao provedor de LLM (minimização de PII).
- O **texto do comentário anterior** para gatilhos `COMMENT_EDIT` (para que o agente possa comparar antes/depois).
- A **direção do voto** para gatilhos `COMMENT_VOTE_THRESHOLD`.
- O **ID do usuário que acionou** e o **ID do badge** (para gatilhos de badge de moderador).

Todo texto não confiável — corpos de comentários, nomes de autores, títulos de páginas, o próprio documento de diretrizes — é **delimitado** na mensagem de contexto com marcadores como `<<<COMMENT_TEXT>>> ... <<<END>>>`. O prompt do sistema da plataforma instrui o modelo a nunca seguir instruções dentro dessas delimitações. Esta é a defesa contra injeção de prompt da plataforma; você não precisa repeti-la no seu prompt.

### As três caixas de seleção

#### Incluir o comentário pai e respostas anteriores no mesmo tópico

Adiciona:
- O **comentário pai** — ID, autor, texto.
- **Respostas irmãs** — as respostas anteriores ao mesmo pai no mesmo tópico.

Útil para: qualquer agente que responda a um comentário em contexto (recepcionistas de boas-vindas, sumarizadores de threads, moderadores lendo respostas em conversas).

Custo: pequeno a médio. Limitado pela quantidade de irmãos que existem em um determinado tópico.

#### Incluir o fator de confiança do comentarista, idade da conta, histórico de banimentos e comentários recentes

Adiciona o bloco **AUTHOR_HISTORY**:

- **Idade da conta em dias** desde o cadastro.
- **Fator de confiança (0-100)** — a pontuação do FastComments que resume quão confiável o usuário é neste site. Veja a página [Detecção de Spam](/guide-moderation.html#spam-detection) no guia de moderação.
- **Contagem de banimentos anteriores.**
- **Total de comentários neste site.**
- **Contagem de conteúdo duplicado** — se o usuário postou texto idêntico recentemente (sinal anti-spam).
- **Sinal de contas cruzadas pelo mesmo IP** — contagem de comentários a partir do mesmo IP sob outras contas (sinal de contas alternativas). O hash do IP em si nunca é enviado ao LLM.
- **Comentários recentes** — até 5 dos comentários mais recentes do usuário, cada um truncado para 300 caracteres, delimitados como texto não confiável.

Útil para: qualquer agente de moderação. Sem isso, o modelo bane contas novas e usuários de longa data de boa-fé com a mesma postura.

Custo: médio. Comentários recentes adicionam a maior quantidade de tokens.

#### Incluir título da página, subtítulo, descrição e meta tags

Adiciona o bloco **PAGE_CONTEXT** — título, subtítulo, descrição e quaisquer meta tags que o FastComments tenha capturado para a página.

Útil para: recepcionistas de boas-vindas e sumarizadores de threads, onde saber sobre o que é a página melhora substancialmente a qualidade da saída.

Custo: pequeno.

### Diretrizes da comunidade

O quarto campo, **Diretrizes da comunidade**, é um bloco de política em texto livre incluído na mensagem de contexto com função de usuário em toda execução, delimitado como texto não confiável da mesma forma que corpos de comentários e outros conteúdos fornecidos pelo usuário. O agente o lê como texto de política, mas a plataforma não o trata como uma instrução de sistema. Veja [Diretrizes da comunidade](#community-guidelines) para o que incluir nele.

### Adicionando contexto seletivamente

Essas caixas de seleção se aplicam por agente, não globalmente. Um padrão comum:

- Recepcionista de boas-vindas: contexto da página **ativado**, contexto de thread **desativado**, histórico do usuário **desativado**.
- Moderador: contexto de thread **desativado**, histórico do usuário **ativado**, contexto da página **desativado**.
- Sumarizador de thread: contexto de thread **ativado**, contexto da página **ativado**, histórico do usuário **desativado**.

Busque o mínimo de contexto que um agente precisa para estar correto nas chamadas que ele realmente faz — contexto extra custa tokens em toda execução, mesmo quando o agente não o utiliza.