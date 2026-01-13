[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Definir `noNewRootComments` como `true` fará com que o widget oculte a área de resposta raiz, mas ainda permitir que os usuários respondam
a comentários filhos. Você poderia, por exemplo, definir isso condicionalmente ao carregar a página para permitir que apenas alguns usuários deixem comentários de nível superior.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---