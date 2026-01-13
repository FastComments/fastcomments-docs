[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments permite que usuários façam upload de imagens. Quando um usuário clica nessa imagem, o FastComments, por padrão,
abrirá uma nova aba para mostrar a imagem em tamanho completo. Definir essa flag como true desativa esse comportamento:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Se você não pretende capturar o clique na imagem por conta própria (veja [onImageClicked](#callbacks)), recomendamos que isso seja combinado com algum styling
para remover a aparência de que a imagem pode ser clicada.

---