Kommentare können mit der folgenden Beispielsyntax durchsucht werden:

- Unscharfe Wortsuche: `cats love`
- Exakte Phrasensuche: `I love cats.`
- Exakte vollständige Kommentarübereinstimmung: `exact="I love cats."`
  - Treffer nur für Kommentare, deren gesamter Text genau diesem Wert entspricht (case-sensitive), und nicht für Kommentare, die ihn lediglich enthalten.
- Nach Seitentitel: `page:"Page Title"`
  - Unterstützt Autovervollständigung.
- Nach Seiten-URL: `page:"https://example.com/some-page"`
  - Unterstützt Autovervollständigung.
- Nach Site/Domain: `site:mysite.com` or `domain:othersite.com`
- Nach Benutzer: `user:"Bob"`
  - Unterstützt Autovervollständigung.

Sie können Suchergebnisse mit anderen Moderatoren oder Administratoren teilen, indem Sie die Seiten-URL von der Moderationsseite freigeben. Der Wert des Suchfelds wird in die URL in Ihrem Browser aufgenommen, nachdem Sie auf "Go" geklickt haben.