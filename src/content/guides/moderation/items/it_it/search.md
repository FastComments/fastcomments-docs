I commenti possono essere cercati con la seguente sintassi di esempio:

- Ricerca fuzzy per parole: `cats love`
- Corrispondenza esatta di frase: `I love cats.`
- Corrispondenza esatta del commento completo: `exact="I love cats."`
  - Corrisponde solo ai commenti il cui testo intero è esattamente questo valore (distinguendo tra maiuscole e minuscole), invece che ai commenti che lo contengono soltanto.
- Per titolo della pagina: `page:"Page Title"`
  - Supporta l'autocompletamento.
- Per URL della pagina: `page:"https://example.com/some-page"`
  - Supporta l'autocompletamento.
- Per sito/dominio: `site:mysite.com` or `domain:othersite.com`
- Per utente: `user:"Bob"`
  - Supporta l'autocompletamento.

Puoi condividere i risultati della ricerca con altri moderatori o amministratori condividendo l'URL della pagina dalla pagina di moderazione. Il valore del campo di ricerca verrà incluso nell'URL del tuo browser dopo che avrai cliccato su "Go".