Tukaj je nekaj simptomov, s katerimi se pogosto srečujemo, in pogoste rešitve.

### Sporočilo "To je demo"

To se prikaže, ko ste kopirali kodo pripomočka z naše domače strani, ki uporablja naš demo
najemnik. Za uporabo vašega najemnika kopirajte kodo pripomočka [od tukaj](https://fastcomments.com/auth/my-account/get-acct-code).

### Napaka "FastComments se ne more naložiti na tej domeni"

FastComments mora vedeti, katere domene so v vaši lasti, da lahko avtenticira zahteve, povezane
z vašim računom. [Oglejte si našo dokumentacijo](/guide-multiple-sites.html#add-domains-to-account), da vidite, kako
odpraviti to napako (preprosto dodajte natančno poddomeno + domeno v vaš račun).

Upoštevajte, da bi se to moralo zgoditi šele po preteku preizkusnega obdobja. Med preizkusnim obdobjem se bodo vse zahteve z novih domen
samodejno dodale v vaš račun.

### Preseljeni komentarji se ne prikazujejo za prilagojene namestitve

Običajno se to zgodi, ko so uvoženi komentarji vezani na `Page ID`, vi pa posredujete URL
(ali nobene vrednosti, v tem primeru se uporabi URL strani).

To lahko razhroščujete z [izvozom vaših komentarjev](https://fastcomments.com/auth/my-account/manage-data/export) in ogledom stolpca `URL ID` (trenutno stolpec `B`).

Zagotovite, da so vrednosti, ki jih vidite v stolpcu `URL ID`, enake vrednostim, ki jih posredujete konfiguraciji pripomočka
kot parameter `urlId`.

Za dodatno razlago poskusite prebrati našo [dokumentacijo Kako so komentarji vezani na strani in članke](/guide-customizations-and-configuration.html#url-id).

Če vse drugo odpove, [se obrnite na nas](https://fastcomments.com/auth/my-account/help).

### Pripomoček za komentarje se ne prikazuje

Če se pripomoček za komentarje ne prikazuje, preverite razvojno konzolo Chrome za napake.

Za večino napačnih konfiguracij bo pripomoček za komentarje vsaj prikazal napako na strani, če se
lahko naloži. Ne videti ničesar običajno kaže na skriptno napako.

### Želena konfiguracija ne deluje po pričakovanjih

Preizkusite našo [razširitev za Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), da vidite, katera
konfiguracija se posreduje pripomočku za komentarje. Če vse odpove, naredite posnetek zaslona, kaj pravi razširitev chrome,
in [se obrnite na nas](https://fastcomments.com/auth/my-account/help).

### Komentarji manjkajo na istem URL z različnim hash bangom

Privzeto bo FastComments uporabil URL strani za "vedro", kjer se shranjujejo komentarji. Če vaši URL-ji vključujejo `#hashbange`, in ti `#hashbangi`
ne bi smeli biti del identifikatorja, ki identificira nit komentarjev, lahko preprosto ignoriramo vrednost hash bang, na primer:

[inline-code-attrs-start title = 'Primer ignoriranja hash bangov'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Upoštevajte, da bo po tej spremembi za obstoječe komentarje potrebna migracija. [Za to se obrnite na nas.](https://fastcomments.com/auth/my-account/help)

### Parametri poizvedbe URL vplivajo na pripomoček

Privzeto bo FastComments uporabil URL strani za "vedro", kjer se shranjujejo komentarji. Če vaši URL-ji vključujejo parametre poizvedbe,
ki ne bi smeli biti del identifikatorja, ki identificira nit komentarjev, jih lahko preprosto ignoriramo, na primer:

[inline-code-attrs-start title = 'Ignoriraj parametre poizvedbe'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Upoštevajte, da bo po tej spremembi za obstoječe komentarje potrebna migracija. [Za to se obrnite na nas.](https://fastcomments.com/auth/my-account/help)

### Ne prejemate e-pošte

Pri FastComments vlagamo veliko truda, da zagotovimo, da je dostava naše e-pošte čim bolj zanesljiva.
Vendar pa je nekaterim ponudnikom e-pošte notorično težko zanesljivo dostavljati. Preverite svojo mapo
spam za sporočila od fastcomments.com.

Če [se obrnete na nas](https://fastcomments.com/auth/my-account/help), vam lahko običajno zagotovimo
več vpogleda, zakaj morda ne vidite e-pošte od nas.
