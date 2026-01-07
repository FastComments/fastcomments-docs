FastComments koristi kombinaciju Markdowna i jednostavnog podskupa HTML-a kako bi omogućio pisanje komentara.

Dobar izvor za [Markdown sintaksu je ovdje](https://www.markdownguide.org/cheat-sheet/).

Za najčešće formatiranje možete koristiti alatnu traku za formatiranje za stvari poput podebljanja ili kurziva teksta.

Neki primjeri sintakse su:

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

Linkovi se mogu pisati kao sirovi URL, bez HTML-a ili druge sintakse, i prema zadanim postavkama link će biti pretvoren u klikabilni link s `target="nofollow noopener"` kako bi se obeshrabrili spameri. Neke stranice mogu odabrati onemogućiti automatsko stvaranje linkova.

Numerirani popisi mogu se pisati ovako:

```
1. Prva stavka.
2. Druga stavka.
3. Treća stavka.
```

Isto vrijedi za popise s grafičkim oznakama:

```
- Neka točka.
- Neka druga točka.
```

Za zajednice orijentirane na programiranje, kod se može dijeliti lijepljenjem u područje komentara, a jezik će biti automatski prepoznat i formatiran. Kod se može dodati u `<code></code>` oznake ili Markdown blokove koda s gravisima.
