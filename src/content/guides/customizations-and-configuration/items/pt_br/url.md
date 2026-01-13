---
[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Ao enviar e-mails de notificação, ou ao renderizar comentários em interfaces de usuário como a página de moderação, é útil poder vincular
do comentário para a página em que ele se encontra.

Se o ID da URL nem sempre for um ID, então precisamos armazenar a URL em outro lugar. É para isso que serve a propriedade "url", definida da seguinte forma.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Um caso de uso comum é vincular a thread de comentários a um identificador, como um artigo, e então fornecer um link de volta para uma página específica, por exemplo:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

A URL não é limpa dos parâmetros de marketing comuns. Por padrão, qualquer que seja a URL da página atual, é essa URL que é armazenada com o comentário.

---