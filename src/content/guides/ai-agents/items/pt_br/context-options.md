A seção **Contexto** no formulário de edição controla quanta informação o agente recebe em cada execução. Mais contexto produz decisões melhores, mas aumenta o custo em tokens por execução, então você deve incluir apenas o que o agente realmente precisa.

### O que é sempre incluído

Mesmo com todas as caixas desmarcadas, a mensagem de contexto do agente inclui:

- O **tipo de evento acionador** (por exemplo, `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- A **URL da página e o ID da URL** (quando conhecidos).
- O **comentário** que acionou a execução, se houver — ID, ID do usuário autor, nome de exibição do autor, texto do comentário, contagens de votos, contagem de sinalizações, flags de spam/aprovado/revisado, ID do pai. O e-mail do autor **nunca** é enviado ao provedor LLM (minimização de PII).
- O **texto anterior do comentário** para gatilhos `COMMENT_EDIT` (para que o agente possa comparar antes/depois).
- A **direção do voto** para gatilhos `COMMENT_VOTE_THRESHOLD`.
- O **ID do usuário que acionou** e o **ID do badge** (para gatilhos de badge de moderador).
- O **catálogo de badges** do seu tenant (nome, rótulo de exibição, descrição) quando o agente tem permissão para conceder badges, para que o agente possa escolher um apropriado sem que você precise detalhar os badges no prompt.

Todo texto não confiável — corpos de comentários, nomes de autores, títulos de páginas, o próprio documento de diretrizes — é **delimitado** na mensagem de contexto com marcadores como `<<<COMMENT_TEXT>>> ... <<<END>>>`. O prompt de sistema da plataforma instrui o modelo a nunca seguir instruções dentro dessas delimitações. Esta é a defesa da plataforma contra prompt-injection; você não precisa repeti-la no seu prompt.

### As três caixas de seleção

#### Incluir o comentário pai e respostas anteriores no mesmo tópico

Adiciona:
- O **comentário pai** - ID, autor, texto.
- **Respostas irmãs** - as respostas anteriores ao mesmo pai no mesmo tópico.

Útil para: qualquer agente que responda a um comentário com contexto (recepcionistas de boas-vindas, sumarizadores de tópicos, moderadores que leem respostas em conversas).

Custo: pequeno a médio. Limitado pelo número de respostas irmãs que existem em um determinado tópico.

#### Incluir fator de confiança do comentarista, idade da conta, histórico de banimentos e comentários recentes

Adiciona o bloco **AUTHOR_HISTORY**:

- **Account age in days** desde o cadastro.
- **Trust factor (0-100)** - a pontuação do FastComments que resume o quão confiável o usuário é neste site. Veja a página [Detecção de Spam](/guide-moderation.html#spam-detection) no guia de moderação.
- **Prior ban count.**
- **Total comments on this site.**
- **Duplicate-content count** - se o usuário postou texto idêntico recentemente (sinal anti-spam).
- **Same-IP cross-account signal** - contagem de comentários do mesmo IP em outras contas (sinal de contas alternativas). O hash do IP em si nunca é enviado ao LLM.
- **Recent comments** - até 5 dos comentários mais recentes do usuário, cada um truncado a 300 caracteres, delimitados como texto não confiável.

Útil para: qualquer agente de moderação. Sem isso, o modelo bane contas novas e usuários de longa data que agem de boa-fé com a mesma postura.

Custo: médio. Comentários recentes adicionam a maior parte dos tokens.

#### Incluir título da página, subtítulo, descrição e meta tags

Adiciona o bloco **PAGE_CONTEXT** - título, subtítulo, descrição e quaisquer meta tags que o FastComments tenha capturado para a página.

Útil para: recepcionistas de boas-vindas e sumarizadores de tópicos, onde saber sobre o que é a página melhora substancialmente a qualidade da saída.

Custo: pequeno.

### Diretrizes da comunidade

O quarto campo, **Community guidelines**, é um bloco de política em texto livre incluído na mensagem de contexto do papel de usuário em cada execução, delimitado como texto não confiável da mesma forma que corpos de comentários e outros conteúdos fornecidos pelo usuário. O agente o lê como texto de política, mas a plataforma não o trata como uma instrução de sistema. Veja [Diretrizes da comunidade](#community-guidelines) para o que colocar nele.

### Adicionando contexto seletivamente

Essas caixas se aplicam por agente, não globalmente. Um padrão comum:

- Recepcionista de boas-vindas: contexto da página **ligado**, contexto do tópico **desligado**, histórico do usuário **desligado**.
- Moderador: contexto do tópico **desligado**, histórico do usuário **ligado**, contexto da página **desligado**.
- Sumarizador de tópicos: contexto do tópico **ligado**, contexto da página **ligado**, histórico do usuário **desligado**.

Mire no mínimo de contexto que um agente precisa para estar correto nas chamadas que ele realmente faz — contexto extra custa tokens em cada execução, mesmo quando o agente não o utiliza.