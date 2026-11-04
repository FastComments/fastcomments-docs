### Pregled

FastComments Image Chat proširuje standardni FastComments widget za komentare, tako da nasleđuje sve opcije konfiguracije osnovnog widgeta, uz dodavanje nekoliko specifičnih za anotacije slika.

### Obavezna konfiguracija

#### tenantId

Potreban je vaš FastComments Tenant ID. Možete ga pronaći u svom [FastComments dashboard-u](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Opcije specifične za Image Chat

#### urlId

Podrazumevano, Image Chat generiše jedinstveni identifikator za svaku konverzaciju na osnovu URL-a stranice, izvora slike i X/Y koordinata. Ovo možete zameniti prilagođenim `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ovo je korisno kada se struktura vašeg URL-a može promeniti, ali želite da zadržite iste konverzacije, ili kada želite da delite anotacije na više stranica.

#### chatSquarePercentage

Kontrolira veličinu klikabilnih markera za ćaskanje kao procenat širine slike. Podrazumevana vrednost je 5%, što znači da je svaki marker 5% širine slike.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Veći, uočljiviji markeri
});
```

Manje vrednosti stvaraju manje uočljive markere koji bolje funkcionišu za detaljne slike. Veće vrednosti čine markere lakšim za viđenje i klikanje na zagušenim slikama ili za korisnike na mobilnim uređajima.

#### hasDarkBackground

Omogućite stil tamnog režima kada vaša stranica ima tamnu pozadinu.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Callback funkcija koja se aktivira svaki put kada se broj komentara promeni. Ovo je korisno za ažuriranje UI elemenata poput bedževa ili naslova stranice.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Nasleđene opcije konfiguracije

Pošto Image Chat proširuje standardni widget za komentare, možete koristiti bilo koju opciju konfiguracije iz osnovnog FastComments widgeta. Evo nekih često korišćenih opcija:

#### locale

Postavite jezik za UI widgeta. FastComments podržava desetine jezika.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Španski
});
```

#### readonly

Učinite sve konverzacije samo za čitanje. Korisnici mogu videti postojeće markere i diskusije, ali ne mogu kreirati nove niti odgovarati.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso i simpleSSO

Integrišite se sa svojim sistemom autentifikacije korišćenjem Single Sign-On-a.

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

Kontrolirajte koliko nivoa duboko odgovori mogu ići. Podrazumevano, Image Chat postavlja ovo na 0, što znači da su svi komentari ravni (bez ugnježdenih odgovora). Ovo možete promeniti ako želite razgranate konverzacije.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dozvoli 3 nivoa ugnježđivanja
});
```

### Interna konfiguracija

Ove opcije postavlja automatski Image Chat i ne bi trebalo da ih zamenjujete:

`productId` se automatski postavlja na `2` za Image Chat. `floating-chat` ekstenzija se automatski učitava kako bi obezbedila funkcionalnost prozora za ćaskanje. Widget automatski detektuje mobilne uređaje (ekrani širi od 768px) i prilagođava UI koristeći prozore za ćaskanje preko celog ekrana.

### Fleksibilnost ciljnog elementa

Prvi parametar za `FastCommentsImageChat` može biti ili `<img>` element direktno ili kontejnerski element sa slikom unutra:

```javascript
// Direktan element slike
FastCommentsImageChat(document.getElementById('my-image'), config);

// Kontejner sa slikom unutra
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Widget će automatski pronaći sliku ako prosledite kontejnerski element.

### Kompletan primer

Evo primera koji prikazuje više opcija konfiguracije zajedno:

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

Za potpunu listu svih dostupnih opcija konfiguracije nasleđenih iz osnovnog widgeta, pogledajte glavnu FastComments dokumentaciju konfiguracije.