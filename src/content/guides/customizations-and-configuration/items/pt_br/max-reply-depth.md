[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Por padrão, o FastComments permite aninhamento ilimitado de respostas, criando uma estrutura de thread onde os usuários podem responder a respostas indefinidamente.

A opção maxReplyDepth permite limitar o quão profundas as threads de respostas podem ser. Quando a profundidade máxima é atingida, os usuários não verão mais o botão de resposta nos comentários nesse nível.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Com maxReplyDepth definido como 2:
- Usuários podem comentar no nível superior (depth 0)
- Usuários podem responder a comentários de nível superior (depth 1)
- Usuários podem responder a essas respostas (depth 2)
- Não são permitidas mais respostas além da profundidade 2

Definir como 1 permitiria apenas respostas a comentários de nível superior, criando uma estrutura de discussão mais rasa.

Definir maxReplyDepth como 0 desabilitaria todas as respostas, permitindo apenas comentários de nível superior. Se não especificado, as respostas podem ser aninhadas sem limite.