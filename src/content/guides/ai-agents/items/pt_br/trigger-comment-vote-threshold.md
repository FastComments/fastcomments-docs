Dispara quando a contagem líquida de votos de um comentário atinge o limite configurado. Votos líquidos são `votesUp - votesDown`.

### Required configuration

- **Limiar de votos** - inteiro >= 1. O gatilho dispara no voto que faz os votos líquidos chegarem exatamente a esse número.

Se o limiar for 10 e um comentário passar de 9 para 10 votos líquidos, o gatilho dispara uma vez. Se um voto então o levar de 10 para 11, o gatilho **não** dispara novamente — ele não dispara em cada voto adicional além do limiar.

### Context the agent receives

- O comentário, com as contagens de votos atuais.
- A **direção do voto** (`up` ou `down`) do voto que acionou a travessia do limiar.
- Histórico opcional do thread / usuário / contexto da página conforme configurado.

### Notable

- Um comentário que sobe para 10, cai para 9 e volta para 10 disparará o gatilho duas vezes. Não existe um estado por comentário de "disparado uma vez" — se você precisar dessa semântica, faça com que o agente registre uma [memory note](#tools-overview) na primeira execução e verifique-a nas execuções subsequentes.
- O limiar é sempre uma contagem de votos **líquida**, não apenas upvotes brutos. Um comentário com 12 up e 2 down tem 10 líquido e dispara o gatilho; um com 10 up e 0 down também dispara.
- Travessias somente por down-vote são possíveis — um comentário indo de 11 para 10 por causa de um down-vote também dispara. O parâmetro `voteDirection` no contexto informa ao agente de qual direção veio a travessia do limiar.

### Common uses

- **Fixar** - o [Top Comment Pinner template](#template-top-comment-pinner) é construído em torno deste gatilho.
- **Promoção / fluxos de comentários em destaque** - emita um evento via [Webhooks](#webhooks-overview) para que um sistema externo possa promover o comentário em outro lugar do seu site.
- **Rastreamento de engajamento** - registre uma memory note sobre o usuário que escreveu o comentário para que outros agentes saibam que ele produziu conteúdo popular.

### Tuning

O limiar adequado é específico da comunidade. Observe o [Run History](#run-history) por alguns dias com um limiar baixo (5) para ver com que frequência ele dispara. Aumente o limiar até que a taxa de disparos corresponda à cadência que você realmente deseja.