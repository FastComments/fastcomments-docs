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
    <!-- API Klijent -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Osnovna biblioteka (uključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub biblioteka (za live događaje) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorijum u vaš build.gradle fajl:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Klijent
    implementation "com.fastcomments:client:2.0.0"
    
    // Osnovna biblioteka (uključuje SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub biblioteka (za live događaje)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Ova biblioteka sadrži tri modula. Generisani API klijent, osnovna Java biblioteka koja sadrži ručno napisane alate za olakšavanje rada sa API‑jem, i `pubsub` modul koji je biblioteka za pretplatu na protoke promena.

- [Dokumentacija API klijenta biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući primere SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Za API klijenta, postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozivati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.

`ModerationApi` pruža opsežan skup live i brzih moderacijskih API‑ja. Svaka metoda `ModerationApi` prima `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.