### Pregled

FastComments Image Chat razširja standardni widget za komentarje FastComments, zato podeduje vse konfiguracijske možnosti osnovnega widgeta in doda nekaj specifičnih za slikovne pripombe.

### Zahtevana konfiguracija

#### tenantId

Vaš FastComments Tenant ID je obvezen. Najdete ga v svoji [FastComments nadzorni plošči](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Možnosti, specifične za Image Chat

#### urlId

Privzeto Image Chat ustvari enolični identifikator za vsako pogovor na podlagi URL-ja strani, vira slike ter koordinat X/Y. To lahko preglasite s svojim `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

To je uporabno, kadar se struktura vaših URL-jev lahko spremeni, a želite ohraniti iste pogovore, ali kadar želite pripombe deliti med več stranmi.

#### chatSquarePercentage

Nadzoruje velikost klikabilnih označevalcev pogovora kot odstotek širine slike. Privzeta vrednost je 5 %, kar pomeni, da je vsak označevalec 5 % širine slike.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Večji, bolj vidni označevalci
});
```

Manjše vrednosti ustvarijo manj moteče označevalce, ki so bolj primerni za podrobne slike. Večje vrednosti naredijo označevalce lažje vidne in klikljive na gosto poslikanih straneh ali za uporabnike na mobilnih napravah.

#### hasDarkBackground

Omogočite temen slog, kadar ima vaša stran temno ozadje.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Povratna funkcija, ki se sproži vsakič, ko se spremeni število komentarjev. Uporabna je za posodabljanje elementov vmesnika, kot so značke ali naslovi strani.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Podedovane konfiguracijske možnosti

Ker Image Chat razširja standardni widget za komentarje, lahko uporabite katero koli konfiguracijsko možnost osnovnega widgeta FastComments. Tukaj je nekaj pogosto uporabljenih možnosti:

#### locale

Nastavite jezik vmesnika widgeta. FastComments podpira ducate jezikov.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Španščina
});
```

#### readonly

Naredite vse pogovore samo za branje. Uporabniki si lahko ogledajo obstoječe označevalce in razprave, ne morejo pa ustvarjati novih ali odgovarjati.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso in simpleSSO

Integrirajte se s svojim sistemom avtentikacije z uporabo enotne prijave (Single Sign-On).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Konfiguracija SSO
    }
});
```

Za podrobnosti o možnostih avtentikacije si oglejte dokumentacijo SSO.

#### maxReplyDepth

Nadzorujte, koliko nivojev globoko lahko gredo odgovori. Privzeto Image Chat nastavi to vrednost na 0, kar pomeni, da so vsi komentarji ravni (brez ugnezdenih odgovorov). To lahko spremenite, če želite razvejane pogovore.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dovoli 3 nivoje ugnezditev
});
```

### Notranja konfiguracija

Te možnosti nastavi Image Chat samodejno in jih ne smete preglasiti:

`productId` je samodejno nastavljen na `2` za Image Chat. Razširitev `floating-chat` se samodejno naloži in zagotovi funkcionalnost okna pogovora. Widget samodejno zazna mobilne naprave (zaslone širše manj kot 768px) in ustrezno prilagodi vmesnik s celozaslonskimi okni pogovora.

### Prilagodljivost ciljnega elementa

Prvi parameter funkcije `FastCommentsImageChat` je lahko neposredno element `<img>` ali vsebnik s sliko znotraj:

```javascript
// Neposreden element slike
FastCommentsImageChat(document.getElementById('my-image'), config);

// Vsebnik s sliko znotraj
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Če podate element vsebnika, bo widget sliko našel samodejno.

### Popoln primer

Tukaj je primer, ki prikazuje več konfiguracijskih možnosti skupaj:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Vaša SSO konfiguracija
    },
    maxReplyDepth: 1
});
```

Za popoln seznam vseh razpoložljivih konfiguracijskih možnosti, podedovanih iz osnovnega widgeta, si oglejte glavno dokumentacijo konfiguracije FastComments.