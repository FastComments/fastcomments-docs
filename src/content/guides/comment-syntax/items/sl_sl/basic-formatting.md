FastComments uporablja kombinacijo Markdowna in preprostega podmnožja HTML-ja za omogočanje pisanja komentarjev.

Dober vir za [sintakso Markdown je tukaj](https://www.markdownguide.org/cheat-sheet/).

Za najpogostejše oblikovanje lahko uporabite orodno vrstico za oblikovanje za stvari, kot sta krepko ali poševno besedilo.

Nekaj primerov sintakse:

- Krepko:
  - `**krepko**`
  - `<b>krepko besedilo</b>`
- Poševno:
  - `*poševno*`
  - `<i>poševno besedilo</i>`
- Prečrtano:
  - `~~prečrtano~~`
  - `<strike>prečrtano</strike>`
- Citati:
  - `> citat`

Upoštevajte, da naslovi Markdown niso podprti.

Povezave se lahko napišejo kot surov URL, brez HTML-ja ali druge sintakse, in privzeto bo povezava pretvorjena v klikljivo povezavo s `target="nofollow noopener"` za odvračanje pošiljateljev neželene pošte. Nekatera spletna mesta se lahko odločijo onemogočiti samodejno ustvarjanje povezav.

Oštevilčene sezname lahko napišete tako:

```
1. Prva postavka.
2. Druga postavka.
3. Tretja postavka.
```

Enako velja za označene sezname:

```
- Neka točka.
- Neka druga točka.
```

Za skupnosti, usmerjene v programiranje, se koda lahko deli z lepljenjem v območje komentarjev, jezik pa bo samodejno zaznan in oblikovan. Kodo lahko dodate v oznake `<code></code>` ali bloke kode Markdown z obrnjenim narekovajem.
