O campo **Diretrizes da comunidade** no formulário de edição é um bloco de texto de política opcional incluído na mensagem de contexto com papel de usuário em cada execução para este agente. Ele é cercado como texto não confiável (o mesmo cercamento que a plataforma aplica aos corpos de comentários e a outros conteúdos fornecidos pelo usuário), então o modelo o trata como referência de política, não como instruções de sistema. É o lugar canônico para registrar "qual comportamento é e não é permitido neste site" para que o agente o aplique de forma consistente.

### Como difere do prompt inicial

- **Prompt inicial** - o papel do agente e o estilo de tomada de decisão. "Você é um moderador. Prefira advertência a banimento."
- **Diretrizes da comunidade** - as regras da sua comunidade, em linguagem de política. "Sem ataques pessoais. Sem links promocionais de contas com menos de 24 horas. Comentários fora do tópico podem ser removidos se um tópico estiver acalorado."

Ambos fluem para a mesma janela de contexto, mas entram em camadas diferentes - o prompt inicial faz parte do papel do sistema, o documento de diretrizes é texto cercado dentro da mensagem de contexto com papel de usuário. A separação facilita a edição quando você quer atualizar um sem reler o outro.

### O que é um bom documento de diretrizes

Um documento curto, específico e escrito por um humano. Listas funcionam melhor que prosa:

[inline-code-attrs-start title = 'Exemplo de Diretrizes da Comunidade'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

O agente aplica isso em cada execução. Se você alterar as diretrizes, a alteração entra em vigor no próximo gatilho - execuções passadas não são reavaliadas retroativamente.

### O que não colocar aqui

- **Instruções de formatação de saída** ("responda em HTML", "use emoji"). Isso pertence ao [prompt inicial](#personality-prompt).
- **Texto localizado.** O documento de diretrizes, assim como o prompt, é **somente em inglês** pelo mesmo motivo - a tradução automática pode alterar o comportamento do agente silenciosamente. Se você tiver políticas que variam por local, escreva todas elas em inglês neste único documento e estruture o documento como "para páginas em alemão: ..."
- **Citações longas de políticas externas.** Parafraseie. Contexto longo consome tokens em toda execução.
- **PII ou segredos.** Este texto é enviado ao provedor LLM em cada execução.

### Tamanho

O campo é limitado a **4000 caracteres** (aplicado tanto pelo formulário quanto pela rota de salvamento). O custo de tokens em cada execução é proporcional ao tamanho, então mesmo dentro do limite algumas centenas de palavras geralmente são suficientes. Se suas políticas comunitárias tiverem muitas páginas, resuma as partes que o agente precisa neste campo.

### Versionamento

Não há histórico de versão embutido para o documento de diretrizes - o último valor salvo é o que o agente usa. Se você quiser histórico, copie o documento para seu próprio sistema de controle antes de cada edição importante. O fluxo [Refine Prompts](#refining-prompts) pode registrar mudanças no *prompt inicial* mas não versiona o documento de diretrizes.

---