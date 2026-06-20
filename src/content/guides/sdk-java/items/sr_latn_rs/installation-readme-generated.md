### Maven

Dodajte Repsy repozitorijum u POM vašeg projekta:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Zatim dodajte zavisnosti koje su vam potrebne:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorijum u vašu build.gradle datoteku:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Sadržaj biblioteke

Ova biblioteka sadrži tri modula. Generisani API klijent, core Java biblioteka koja sadrži ručno napisane alatke koje olakšavaju rad sa API-jem, i `pubsub` modul koji je biblioteka za pretplatu na feed-ove promena.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija Core biblioteke, uključujući SSO primere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni naspram zaštićenih API-ja

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi`, i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.

`ModerationApi` pokreće moderatorski panel. Sadrži metode za moderaciju komentara (listanje, brojanje, pretraga, zapisi i izvoz), akcije moderacije (uklanjanje/obnavljanje, označavanje, podešavanje statusa za pregled/spam/odobrenje, glasovi, i ponovno otvaranje/zatvaranje niti), zabrane (zabrana komentarisanja, poništavanje zabrane, pre-ban sažeci, status i podešavanja zabrane, i brojevi zabranjenih korisnika), i značke i poverenje (dodeljivanje/uklanjanje značke, manuelne značke, dobijanje/postavljanje faktora poverenja, i interni profil korisnika). Svaka `ModerationApi` metoda prihvata `sso` parametar tako da poziv može biti izvršen u ime moderatora autentifikovanog putem SSO.