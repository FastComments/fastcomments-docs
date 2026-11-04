Disparado quando a contagem líquida de votos de um comentário atinge o limite configurado. Votos líquidos é `votesUp - votesDown`.

### Configuração necessária

- **Limite de votos** - inteiro >= 1. O gatilho dispara no voto que leva os votos líquidos exatamente a esse número.

Se o limite for 10 e um comentário passar de 9 para 10 votos líquidos, o gatilho dispara uma vez. Se um voto então o levar de 10 para 11, o gatilho **não** dispara novamente - ele não dispara a cada voto adicional além do limite.

### Contexto que o agente recebe

- O comentário, com as contagens de votos atuais.
- A **direção do voto** (`up` ou `down`) do voto que provocou o cruzamento do limite.
- Histórico opcional de thread / usuário / contexto da página, conforme configurado.

### Observações importantes

- Um comentário que sobe para 10, volta para 9 e sobe para 10 novamente disparará o gatilho duas vezes. Não há estado de "disparado uma vez" por comentário - se você precisar dessa semântica, faça o agente registrar uma [nota de memória](#tools-overview) na primeira execução e verificar isso nas execuções subsequentes.
- O limite é sempre uma contagem de votos **líquida**, não votos positivos brutos. Um comentário com 12 positivos e 2 negativos tem líquido 10 e dispara o gatilho; um com 10 positivos e 0 negativos também dispara.
- Cruzamentos apenas por voto negativo são possíveis - um comentário que vai de 11 para 10 por causa de um voto negativo também dispara. O parâmetro `voteDirection` no contexto informa ao agente de qual direção veio o cruzamento do limite.

### Usos comuns

- **Fixação** - o [template Top Comment Pinner](#template-top-comment-pinner) é construído em torno deste gatilho.
- **Fluxos de promoção / comentário destacado** - emita um evento via [Webhooks](#webhooks-overview) para que um sistema externo possa promover o comentário em outro lugar do seu site.
- **Acompanhamento de engajamento** - registre uma nota de memória sobre o usuário que escreveu o comentário para que outros agentes saibam que ele produziu conteúdo popular.

### Ajuste

O limite correto é específico de cada comunidade. Observe o [Run History](#run-history) por alguns dias com um limite baixo (5) para ver com que frequência ele dispara. Aumente o limite até que a taxa de disparo corresponda à cadência que você realmente deseja.