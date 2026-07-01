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
    <!-- API klijent -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Core biblioteka (uključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub biblioteka (za događaje uživo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
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
    // API klijent
    implementation "com.fastcomments:client:3.0.0"
    
    // Core biblioteka (uključuje SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub biblioteka (za događaje uživo)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Library Contents

Ova biblioteka sadrži tri modula. Generisani API klijent, core Java biblioteka koja sadrži ručno napisane alate za olakšavanje rada sa API-jem, i `pubsub` modul koji je biblioteka za pretplatu na promenljive feedove.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija core biblioteke, uključujući SSO primere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati direktno iz pretraživača/mobilnog uređaja/itd. bez autentikacije.

`ModerationApi` pruža opsežan paket live i brzih API-ja za moderaciju. Svaka `ModerationApi` metoda prihvata `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.