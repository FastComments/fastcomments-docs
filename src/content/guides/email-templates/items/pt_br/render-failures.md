Como os templates de e-mail suportam variáveis, e lógica, é possível criar templates
que falhem ao renderizar, ou às vezes não sejam renderizados.

Isto pode ser muito frustrante de diagnosticar e depurar, especialmente se for um problema intermitente, ou
se ocorrer apenas quando os dados estão de uma certa forma.

Para ajudar, FastComments Email Templates tem alguns recursos:

1. Se o template falhar ao pré-visualizar, ele não pode ser salvo. Uma mensagem de erro será exibida.
2. Falhas de renderização de templates são rastreadas e reportadas na UI de administração.

O segundo item está descrevendo falhas de renderização que acontecem em produção. Ou seja, você cria um template que pré-visualiza
corretamente - mas depois falha por algum motivo. Por exemplo, se tivermos isto em nosso template:

    <% if (comment.commenterEmail.includes('test') { %>

Isto pode falhar às vezes se tivermos comentários anônimos ativados, já que o e-mail nem sempre estará
disponível. Então como descobrimos isso?

A resposta é que os erros são exibidos em dois lugares. Primeiro, a própria lista de templates
mostra uma contagem de erros de renderização com cada template.

Então, ao visualizar um template, podemos ver uma contagem, por erro, do número de vezes que o template
falhou ao renderizar.

Um botão de reset está localizado ao lado de cada erro, e sua contagem, para que possamos zerar o contador
apos termos resolvido um problema.