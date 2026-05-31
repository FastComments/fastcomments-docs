Os comentários podem ser pesquisados com a seguinte sintaxe de exemplo:

- Fuzzy word search: `cats love`
- Exact phrase match: `I love cats.`
- Exact full-comment match: `exact="I love cats."`
  - Corresponde apenas a comentários cujo texto inteiro é exatamente esse valor (sensível a maiúsculas/minúsculas), em vez de comentários que apenas o contenham.
- By Page Title: `page:"Page Title"`
  - Suporta preenchimento automático.
- By Page URL: `page:"https://example.com/some-page"`
  - Suporta preenchimento automático.
- By Site/Domain: `site:mysite.com` or `domain:othersite.com`
- By User: `user:"Bob"`
  - Suporta preenchimento automático.

Você pode compartilhar os resultados da pesquisa com outros moderadores ou administradores compartilhando a URL da página a partir da página de moderação. O valor do campo de pesquisa será incluído na URL no seu navegador depois que você clicar em "Go".