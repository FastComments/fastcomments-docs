Kommentarer kan søges med følgende eksempelsyntaks:

- Tilnærmet ordsøgning: `cats love`
- Præcis frasematch: `I love cats.`
- Præcis match for hele kommentaren: `exact="I love cats."`
  - Matcher kun kommentarer, hvis hele teksten er nøjagtigt denne værdi (skelner mellem store og små bogstaver), i stedet for kommentarer, der blot indeholder den.
- Efter sidetitel: `page:"Page Title"`
  - Understøtter autofuldførelse.
- Efter side-URL: `page:"https://example.com/some-page"`
  - Understøtter autofuldførelse.
- Efter websted/domæne: `site:mysite.com` eller `domain:othersite.com`
- Efter bruger: `user:"Bob"`
  - Understøtter autofuldførelse.

Du kan dele søgeresultater med andre moderatorer eller administratorer ved at dele side-URL'en fra moderationssiden. Søgefeltets
værdi vil blive inkluderet i URL'en i din browser efter du klikker på "Go".

---