### Pregled

FastComments Image Chat razširja standardni pripomoček za komentiranje FastComments, zato deduje vse možnosti konfiguracije iz osnovnega pripomočka in dodaja nekaj, specifičnih za oznake na slikah.

### Zahtevana konfiguracija

#### tenantId

Za uporabo potrebujete FastComments Tenant ID. Najdete ga v svoji [FastComments nadzorni plošči](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Možnosti, specifične za Image Chat

#### urlId

Privzeto Image Chat ustvari edinstven identifikator za vsak pogovor na podlagi URL-ja strani, vira slike in X/Y koordinat. To lahko presežete z lastnim `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

To je uporabno, kadar se struktura vaših URL-jev lahko spremeni, vendar želite obdržati iste pogovore, ali kadar želite deliti oznake med več stranmi.

#### chatSquarePercentage

Nadzoruje velikost klikabilnih oznak za klepet kot odstotek širine slike. Privzeto je 5 %, kar pomeni, da je vsaka oznaka 5 % širine slike.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Večji, bolj vidni markerji
});
```

Manjše vrednosti ustvarijo manj vsiljive oznake, ki bolje delujejo pri podrobnih slikah. Večje vrednosti naredijo oznake lažje vidne in klikljive na zasedenih slikah ali za uporabnike na mobilnih napravah.

#### hasDarkBackground

Omogoči temno oblikovanje, kadar ima vaša stran temno ozadje.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Funkcija povratnega klica, ki se sproži vsakič, ko se število komentarjev spremeni. To je uporabno za posodabljanje elementov uporabniškega vmesnika, kot so značke ali naslovi strani.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Dedičene možnosti konfiguracije

Ker Image Chat razširja standardni pripomoček za komentiranje, lahko uporabite katerokoli možnost konfiguracije iz osnovnega pripomočka FastComments. Tukaj je nekaj pogosto uporabljenih možnosti:

#### locale

Nastavite jezik za uporabniški vmesnik pripomočka. FastComments podpira desetine jezikov.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // španščina
});
```

#### readonly

Naredite vse pogovore samo za branje. Uporabniki lahko gledajo obstoječe oznake in razprave, vendar ne morejo ustvarjati novih ali odgovarjati.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integrirajte z vašim sistemom za overjanje z uporabo enkratnega prijavljanja (Single Sign-On).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // konfiguracija SSO
    }
});
```

Oglejte si dokumentacijo o SSO za podrobnosti o možnostih overjanja.

#### maxReplyDepth

Nadzoruje, koliko nivojev globoko lahko segajo odgovori. Privzeto je Image Chat nastavil to na 0, kar pomeni, da so vsi komentarji ploski (brez gnezdenih odgovorov). To lahko spremenite, če želite naviti pogovore.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dovoli 3 ravni gnezdenja
});
```

### Notranja konfiguracija

Te možnosti Image Chat samodejno nastavi in jih ne bi smeli preglasiti:

Vrednost `productId` je samodejno nastavljena na `2` za Image Chat. Razširitev `floating-chat` se samodejno naloži, da zagotovi funkcionalnost okna za klepet. Pripomoček samodejno zazna mobilne naprave (zasloni oz. širine pod 768px) in ustrezno prilagodi uporabniški vmesnik z okni za klepet v celozaslonskem načinu.

### Fleksibilnost ciljanega elementa

Prvi parameter za `FastCommentsImageChat` je lahko bodisi neposredni element `<img>`, bodisi element vsebnika s sliko znotraj:

```javascript
// Neposredni element slike
FastCommentsImageChat(document.getElementById('my-image'), config);

// Kontejner s sliko znotraj
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Pripomoček bo samodejno poiskal sliko, če posredujete element vsebnika.

### Celoten primer

Tukaj je primer, ki prikazuje več možnosti konfiguracije skupaj:

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

Za popoln seznam vseh razpoložljivih možnosti konfiguracije, ki se podedujejo iz osnovnega pripomočka, si oglejte glavno dokumentacijo o konfiguraciji FastComments.