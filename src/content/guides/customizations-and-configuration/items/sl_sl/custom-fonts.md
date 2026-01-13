[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je zasnovan tako, da je mogoče opraviti prilagoditve, in pisava, ki jo uporabljajo naši pripomočki, ni izjema.

Privzeto FastComments uporablja `system font stack`, da izgleda čim bolje na širokem naboru naprav.

Za določitev lastnih pisav glejte [Dokumentacija za prilagojen CSS](/guide-customizations-and-configuration.html#custom-css).

Tam boste našli način za definiranje prilagojenega CSS-a, kar vam bo omogočilo nastaviti želene pisave.

#### Kako določiti pisavo

Za preglasitev pisave priporočamo, da svoj CSS definirate z selektorjema `.fast-comments, textarea`. Na primer:

[inline-code-attrs-start title = 'Primer zunanje prilagojene pisave'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---