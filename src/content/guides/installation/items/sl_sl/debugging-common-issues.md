Tukaj je nekaj simptomov, s katerimi se pogosto srečujemo, in pogoste rešitve. 

### Sporočilo "This is a demo"

To se prikaže, ko ste kopirali kodo vtičnika z naše domače strani, ki uporablja naš demo tenant. Če želite uporabljati svoj tenant, kopirajte kodo vtičnika s [te strani](https://fastcomments.com/auth/my-account/get-acct-code).

### Napaka "FastComments cannot load on this domain"

FastComments mora vedeti, katera domenska imena so v vaši lasti, da lahko preveri zahteve, povezane z vašim računom. [Oglejte si našo dokumentacijo](/guide-multiple-sites.html#add-domains-to-account), da vidite, kako odpraviti to napako (preprosto dodajte točno subdomeno + domeno v svoj račun).

Upoštevajte, da se to običajno zgodi šele po koncu poskusnega obdobja. Med poskusnim obdobjem bodo vse zahteve z novih domen samodejno dodane v vaš račun.

### Migrirani komentarji se ne prikazujejo za prilagojene inštalacije

Običajno se to zgodi, ko so uvoženi komentarji vezani na `Page ID`, vi pa posredujete URL (ali nobene vrednosti, v tem primeru privzeto uporabljeno vrednost URL strani).

To lahko odpravite tako, da [izvozite svoje komentarje](https://fastcomments.com/auth/my-account/manage-data/export) in si ogledate stolpec `URL ID` (trenutno stolpec `B`).

Prepričajte se, da so vrednosti, ki jih vidite v stolpcu `URL ID`, enake vrednostim, ki jih posredujete konfiguraciji vtičnika kot parameter `urlId`.

Za dodatno razlago poskusite prebrati našo [dokumentacijo Kako so komentarji vezani na strani in članke](/guide-customizations-and-configuration.html#url-id).

Če vse drugo odpove, nam [pišite](https://fastcomments.com/auth/my-account/help).

### Komentarni vtičnik se ne prikazuje

Če se komentarni vtičnik ne prikaže, preverite Chrome razvojno konzolo za napake.

Pri večini napačnih konfiguracij se bo komentarni vtičnik vsaj prikazal z napako na strani, če se mu uspe naložiti. Če ne vidite ničesar, je to običajno znak skriptne napake.

### Želena konfiguracija ne deluje po pričakovanjih

Poskusite naš [Chrome razširitev](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), da vidite, katera konfiguracija se posreduje komentarjem. Če vse odpove, naredite posnetek zaslona tistega, kar pove razširitev, in nam [pišite](https://fastcomments.com/auth/my-account/help).

### Komentarji manjkajo na istem URL z različnim hash-bangom

Privzeto FastComments uporablja URL strani kot "vedro", kjer so shranjeni komentarji. Če vaši URL-ji vsebujejo `#hashbangs`, in ti `#hashbangs` ne bi smeli biti del identifikatorja, ki določa niti komentarjev, jih lahko preprosto zanemarimo, na primer:

[inline-code-attrs-start title = 'Primer: ignoriranje hash-bangov'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Upoštevajte, da bo po tej spremembi treba izvesti migracijo za obstoječe komentarje. [V ta namen nam pišite.](https://fastcomments.com/auth/my-account/help)

### Vpliv URL parametrov poizvedbe na vtičnik

Privzeto FastComments uporablja URL strani kot "vedro", kjer so shranjeni komentarji. Če vaši URL-ji vsebujejo parametre poizvedbe, ki ne bi smeli biti del identifikatorja niti komentarja, jih lahko preprosto zanemarimo, na primer:

[inline-code-attrs-start title = 'Ignoriranje parametrov poizvedbe'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Upoštevajte, da bo po tej spremembi treba izvesti migracijo za obstoječe komentarje. [V ta namen nam pišite.](https://fastcomments.com/auth/my-account/help)

### Ne prejemate e-pošte

V FastComments vložimo veliko truda, da je dostava naših e-poštnih sporočil čim bolj zanesljiva. Vendar so nekateri ponudniki e-pošte znano težavni za zanesljivo dostavo. Preverite mapo z vsiljeno pošto za sporočila s strani fastcomments.com.

Če nam [pišete](https://fastcomments.com/auth/my-account/help), vam običajno lahko damo več informacij o tem, zakaj morda ne vidite naših e-poštnih sporočil.