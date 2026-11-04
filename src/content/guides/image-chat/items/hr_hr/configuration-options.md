### Pregled

FastComments Image Chat proširuje standardni FastComments widget za komentare, tako da nasljeđuje sve opcije konfiguracije osnovnog widgeta, uz dodavanje nekoliko specifičnih za anotacije slika.

### Obavezna konfiguracija

#### tenantId

Potreban je vaš FastComments Tenant ID. Možete ga pronaći u svom [FastComments dashboardu](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Opcije specifične za Image Chat

#### urlId

Prema zadanim postavkama, Image Chat generira jedinstveni identifikator za svaki razgovor na temelju URL-a stranice, izvora slike te X/Y koordinata. To možete nadjačati prilagođenim `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ovo je korisno kada se struktura vašeg URL-a može promijeniti, ali želite zadržati iste razgovore, ili kada želite dijeliti anotacije na više stranica.

#### chatSquarePercentage

Kontrolira veličinu klikabilnih oznaka razgovora kao postotak širine slike. Zadana vrijednost je 5%, što znači da je svaka oznaka 5% širine slike.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Veće, vidljivije oznake
});
```

Manje vrijednosti stvaraju manje nametljive oznake koje bolje funkcioniraju za detaljne slike. Veće vrijednosti čine oznake lakšima za vidjeti i kliknuti na zauzetim slikama ili za korisnike na mobilnim uređajima.

#### hasDarkBackground

Omogućite stiliziranje tamnog načina rada kada vaša stranica ima tamnu pozadinu.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Funkcija povratnog poziva koja se aktivira svaki put kada se broj komentara promijeni. Ovo je korisno za ažuriranje UI elemenata poput bedževa ili naslova stranica.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Naslijeđene opcije konfiguracije

Budući da Image Chat proširuje standardni widget za komentare, možete koristiti bilo koju opciju konfiguracije iz osnovnog FastComments widgeta. Evo nekoliko često korištenih opcija:

#### locale

Postavite jezik za UI widgeta. FastComments podržava desetke jezika.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Španjolski
});
```

#### readonly

Učinite sve razgovore samo za čitanje. Korisnici mogu pregledavati postojeće oznake i rasprave, ali ne mogu stvarati nove ni odgovarati.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso i simpleSSO

Integrirajte se sa svojim sustavom autentifikacije pomoću Single Sign-On-a.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO konfiguracija
    }
});
```

Pogledajte SSO dokumentaciju za potpune detalje o opcijama autentifikacije.

#### maxReplyDepth

Kontrolirajte koliko razina dubine odgovori mogu ići. Prema zadanim postavkama, Image Chat postavlja ovo na 0, što znači da su svi komentari ravni (bez ugniježđenih odgovora). To možete promijeniti ako želite razgovore s nitima.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dopusti 3 razine ugniježđenja
});
```

### Interna konfiguracija

Ove opcije automatski postavlja Image Chat i ne bi ih trebalo nadjačavati:

`productId` je automatski postavljen na `2` za Image Chat. Ekstenzija `floating-chat` se automatski učitava kako bi omogućila funkcionalnost prozora razgovora. Widget automatski otkriva mobilne uređaje (zaslone širine ispod 768px) i prilagođava UI sukladno tome s fullscreen prozorima razgovora.

### Fleksibilnost ciljanog elementa

Prvi parametar za `FastCommentsImageChat` može biti izravno `<img>` element ili element spremnika sa slikom unutra:

```javascript
// Izravni element slike
FastCommentsImageChat(document.getElementById('my-image'), config);

// Spremnik sa slikom unutra
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Widget će automatski pronaći sliku ako proslijedite element spremnika.

### Potpuni primjer

Evo primjera koji prikazuje više opcija konfiguracije zajedno:

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

Za potpuni popis svih dostupnih opcija konfiguracije naslijeđenih iz osnovnog widgeta, pogledajte glavnu dokumentaciju konfiguracije FastComments.