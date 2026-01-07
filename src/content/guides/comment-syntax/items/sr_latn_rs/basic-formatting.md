FastComments koristi kombinaciju Markdowna i jednostavnog podskupa HTML-a da bi omogućio pisanje komentara.

Dobar resurs za [Markdown sintaksu je ovde](https://www.markdownguide.org/cheat-sheet/).

Za najčešće formatiranje možete koristiti traku alata za formatiranje da radite stvari poput podebljanja ili kurziva teksta.

Neki primeri sintakse su:

- Podebljano:
  - `**podebljano**`
  - `<b>podebljani tekst</b>`
- Kurziv:
  - `*kurziv*`
  - `<i>tekst u kurzivu</i>`
- Precrtano:
  - `~~precrtano~~`
  - `<strike>precrtano</strike>`
- Citati:
  - `> citat`

Imajte na umu da Markdown naslovi nisu podržani.

Linkovi se mogu pisati kao sirovi URL, bez HTML-a ili druge sintakse, i podrazumevano će link biti pretvoren u link na koji se može kliknuti sa `target="nofollow noopener"` da bi se obeshrabrili spameri. Neki sajtovi mogu izabrati da onemoguće automatsko kreiranje linkova.

Numerisane liste mogu se pisati ovako:

```
1. Prva stavka.
2. Druga stavka.
3. Treća stavka.
```

Isto važi za liste sa oznakama:

```
- Neka tačka.
- Neka druga tačka.
```

Za zajednice orijentisane na programiranje, kod se može deliti lepljenjem u oblast komentara, a jezik će biti automatski prepoznat i formatiran. Kod se može dodati u `<code></code>` oznake ili Markdown blokove koda sa gravisima.
